from __future__ import absolute_import, division, print_function

from hexdump import dump as hexdump
from enum import Enum
import struct
import Crypto.Cipher.AES
import Crypto.Util.Counter
from threading import Thread, Event
from functools import partial

from .sink import AlsaSink, Sink
from .decoder.decoder import Decoder


AESIV = "72e067fbddcbcf77ebe8bc643f630d93"

class PlayerState(Enum):
    UNLOADED = 0
    LOADED = 1
    PLAYING = 2
    PAUSED = 3

class SpotifyFile(object):
    CHUNK_SIZE=0x400 # in words

    def __init__(self, session, file_id, key):
        self.session = session
        self.file_id = file_id
        self.key = key
        self.chunks = dict()
        self.signal = Event()

        # in words
        self.cursor = 0
        self.size = None
        # in bytes
        self.extra = 0

    def open(self):
        channel = self.session.channels.create(on_header = self.on_header)
        data = struct.pack('>HbbHLLL20sLL',
                channel,
                0x0, 0x1,
                0x0,
                0x0,
                0x00009C40,
                0x00020000,
                self.file_id,
                0, 1)
        self.session.send_encrypted_packet(0x8, data)

        self.signal.wait()
        self.signal.clear()

    def tell(self):
        return self.cursor * 4 + self.extra

    def seek(self, offset, whence):
        # TODO: negative seeking
        assert offset >= 0
        extra = offset % 4
        offset //= 4

        if whence == 0:
            base = 0
        elif whence == 1:
            base = self.cursor
            extra += self.extra
        elif whence == 2:
            base = self.size
        else:
            assert False

        p = base + offset

        while extra > 4:
            extra -= 4
            p += 1

        if p > self.size:
            p = self.size

        self.cursor = p
        self.extra = extra

    def read(self, size):
        # TODO: non words read, and take extra into account
        assert size % 4 == 0
        size //= 4

        data = bytes()

        while size > 0:
            index = self.cursor // self.CHUNK_SIZE
            offset = self.cursor % self.CHUNK_SIZE
            length = min(size, self.CHUNK_SIZE - offset)

            self.fetch(index)

            d = self.chunks[index][offset*4:(offset+length) * 4]
            l = len(d) // 4

            self.cursor += l
            size -= l
            data += d

            if len(d) < length:
                # EOF
                break

        return data

    def fetch(self, index):
        if index not in self.chunks:
            channel = self.session.channels.create(on_data = partial(self.on_data, index))
            data = struct.pack('>HbbHLLL20sLL',
                    channel,
                    0x0, 0x1,
                    0x0,
                    0x0,
                    0x00009C40,
                    0x00020000,
                    self.file_id,
                    index * self.CHUNK_SIZE,
                    (index + 1) * self.CHUNK_SIZE)
            self.session.send_encrypted_packet(0x8, data)

            self.signal.wait()
            self.signal.clear()

    def on_header(self, ch, header_id, data):
        if header_id == 0x3:
            self.size, = struct.unpack('>L', data)
            self.signal.set()

    def on_data(self, index, ch, data):
        if data is None:
            self.signal.set()
        else:
            assert index not in self.chunks

            ctr = Crypto.Util.Counter.new(
                    128,
                    initial_value=long(AESIV, 16) + index * 0x100)
            key = Crypto.Cipher.AES.new(
                    self.key,
                    Crypto.Cipher.AES.MODE_CTR,
                    counter=ctr)

            data = key.decrypt(data)

            # Weird initial ogg frame that needs fixing
            if index == 0 and ord(data[5]) == 6:
                data = data[:5] + '\2' + data[6:]
            self.chunks[index] = data

class PlayerThread(Thread):
    def __init__(self, session, file_id, key, sink):
        super(PlayerThread, self).__init__()
        self.daemon = True
        self.sink = sink
        self.f = SpotifyFile(session, file_id, key)

    def run(self):
        self.f.open()
        d = Decoder(self.f)
        d.time_seek(0)
        data = d.read(4096)
        while len(data) > 0:
            self.sink.write(data)
            data = d.read(4096)

class Wrapper(object):
    def __init__(self, f):
        self.f = f

    def seek(self, *args):
        print('seek %x %x' % args)
        return self.f.seek(*args)

    def read(self, *args):
        n = args[0]
        data = self.f.read(*args)
        print('read %x %x' % (n,len(data)))
        return data

    def tell(self, *args):
        p = self.f.tell(*args)
        print('tell %x' % p)
        return p

class Player(object):
    def __init__(self, session):
        self.session = session
        self.state = PlayerState.UNLOADED
        self.data = None
        self.key = None
        self.keycount = 0
        self.channel = None
        self.fileid = None
        self.sink = AlsaSink()

    def load(self, track):
        self.state = PlayerState.UNLOADED

        self.data = bytes()
        self.key = None
        self.keycount += 1
        self.fileid = track.file.gid

        data = self.fileid + \
               track.gid + \
               struct.pack('>L', self.keycount) + \
               '\0\0'
        self.session.send_encrypted_packet(0xc,data)

    def play(self):
        t = PlayerThread(
                self.session,
                self.fileid,
                self.key,
                self.sink)
        t.start()

    def handle_aeskey(self, id, data):
        if id != self.keycount:
            return
        self.key = data
        self.state = PlayerState.LOADED

