from __future__ import absolute_import, division, print_function

import struct
from hexdump import dump as hexdump

from .util import protobuf_parse
from . import protocol

PROTOCOLS = {
        'vnd.spotify/mercury-mget-request': protocol.MercuryMultiGetRequest,
        'vnd.spotify/mercury-mget-reply': protocol.MercuryMultiGetReply,
        'vnd.spotify/metadata-track': protocol.Track,
        'vnd.spotify/metadata-artist': protocol.Artist
}

def find_protocol(mime):
    return PROTOCOLS[mime] if mime and mime in PROTOCOLS else None

def decode_payload(data, mime=None, schema=None):
    proto = schema if schema else find_protocol(mime)
    if proto:
        return protobuf_parse(proto, data)
    else:
        return None

class Mercury(object):
    def __init__(self, session):
        self.session = session
        self.callbacks = dict()
        self.subscriptions = dict()
        self.seq = 0

        self.frames = dict()
        self.data = dict()

    def request(self, method, url, payload=None, mime=None, callback=None, schema=None):
        seq, data = self.encode_request(method, url, payload=payload, mime=mime)

        if method == 'SUB':
            cmd = 0xb3
        elif method == 'UNSUB':
            cmd = 0xb4
        else:
            cmd = 0xb2

        self.session.send_encrypted_packet(cmd, data)
        if callback:
            self.callbacks[seq] = (callback, schema)


    def get(self, *args, **kwargs):
        self.request('GET', *args, **kwargs)

    def subscribe(self, url, callback=None, schema=None):
        def cb(response, *subs):
            for s in subs:
                self.subscriptions[s.url] = (callback, schema)

        self.request('SUB', url,
                callback=cb,
                schema=protocol.MercurySubscribed)

    def encode_request(self, method, url, mime=None, payload=None, seq=None):
        if seq is None:
            seq = struct.pack('>L', self.seq)
            self.seq += 1

        flags = 1
        count = 2 if payload else 1

        header = struct.pack('>H', len(seq)) + \
                seq + \
                struct.pack('>bH', flags, count)

        request = protocol.MercuryRequest()
        request.url = url
        request.method = method
        if mime:
            request.mime = mime
        else:
            request.mime = ""

        data = header + \
                struct.pack('>H', request.ByteSize()) + \
                request.SerializeToString()
        if payload:
            data += struct.pack('>H', payload.ByteSize()) + \
                    payload.SerializeToString()

        return seq, data

    def handle_packet(self, cmd, data):
        seq, flags, count, data = self.parse_header(data)
        #print('\'%s\' %x %x %x' % (hexdump(seq), flags, count, len(data)))

        self.data.setdefault(seq, bytes())
        self.frames.setdefault(seq, list())

        frames = self.parse_frames(data, count)
        frames[0] = self.data[seq] + frames[0]

        if flags == 2:
            self.frames[seq] += frames[:-1]
            self.data[seq] = frames[-1]
        else:
            self.frames[seq] += frames
            del self.data[seq]

        if flags != 1:
            return

        frames = self.frames[seq]
        del self.frames[seq]

        response = protobuf_parse(protocol.MercuryReply, frames[0])
        if cmd == 0xb5:
            cs = self.subscriptions.get(response.url)
        else:
            cs = self.callbacks.get(seq)

        if not cs:
            return

        callback, schema = cs
        payloads = map(
                lambda f: decode_payload(f, schema=schema, mime=response.mime),
                frames[1:])

        callback(response, *payloads)
        if cmd != 0xb5:
            del self.callbacks[seq]

    def parse_header(self, data):
        offset = 0
        seq_length, = struct.unpack_from('>H', data, offset)
        offset += 2
        seq = data[offset:offset+seq_length]
        offset += seq_length
        flags, count = struct.unpack_from('>bH', data, offset)
        offset += 3

        return seq, flags, count, data[offset:]

    def parse_frames(self, data, count):
        offset = 0
        frames = []
        for i in range(count):
            length, = struct.unpack_from('>H', data, offset)
            offset += 2
            frames.append(data[offset:offset+length])
            offset += length
        return frames

