from __future__ import absolute_import, division, print_function

from hexdump import dump as hexdump
from enum import Enum
import subprocess as sp
import struct
import Crypto.Cipher.AES
import Crypto.Util.Counter

from .sink import AlsaSink

AESIV = "72e067fbddcbcf77ebe8bc643f630d93"

class PlayerState(Enum):
    UNLOADED = 0
    LOADED = 1
    PLAYING = 2
    PAUSED = 3

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
        self.channel = self.session.channels.create(self.handle_audiodata)
        self.fileid = track.file.gid

        data =  self.fileid + \
                track.gid + \
                struct.pack('>L', self.keycount) + \
                '\0\0'
        self.session.send_encrypted_packet(0xc,data)

    def play(self):
        self.sink.write(self.data)

    def handle_aeskey(self, id, data):
        if id != self.keycount:
            return

        ctr = Crypto.Util.Counter.new(
                128,
                initial_value=long(AESIV, 16))
        self.key = Crypto.Cipher.AES.new(
                data,
                Crypto.Cipher.AES.MODE_CTR,
                counter=ctr)

        data = struct.pack('>HbbHLLL20sLL',
                self.channel,
                0x0, 0x1,
                0x0,
                0x0,
                0x00009C40,
                0x00020000,
                self.fileid,
                0x00000000,
                0xffffffff)

        self.session.send_encrypted_packet(0x8, data)
            

    def handle_audiodata(self, channel, data):
        if channel != self.channel:
            return

        if data is None:
            self.decode()
            self.state = PlayerState.LOADED
        else:
            data = self.key.decrypt(data)
            if len(self.data) == 0 and ord(data[5]) == 6:
                offset = 28
                l = ord(data[26])
                for i in range(l):
                    offset += ord(data[27+i])
                data = data[offset:]

            self.data += data

    def decode(self):
        p = sp.Popen([
            'ffmpeg',
            '-i', '-',
            '-f', 's16le',
            '-acodec', 'pcm_s16le',
            '-ar', '44100',
            '-loglevel', 'panic',
            '-'], stdin = sp.PIPE, stdout = sp.PIPE)
        self.data = p.communicate(self.data)[0]

        assert p.returncode == 0


