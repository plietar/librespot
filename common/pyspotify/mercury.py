from __future__ import absolute_import, division, print_function

import struct
from hexdump import dump as hexdump

from .util import protobuf_parse
from . import protocol

PROTOCOLS = {
        'vnd.spotify/mercury-mget-request': protocol.MercuryMultiGetRequest,
        'vnd.spotify/mercury-mget-reply': protocol.MercuryMultiGetReply,
        'vnd.spotify/metadata-track': protocol.Track
}

def find_protocol(type):
    return PROTOCOLS[type] if type and type in PROTOCOLS else None

def decode_payload(data, type=None, schema=None):
    proto = find_protocol(type) if type else schema
    if proto:
        return protobuf_parse(proto, data)
    else:
        return None

class Mercury(object):
    def __init__(self, session):
        self.session = session
        self.callbacks = dict()
        self.seq = 0

    def get(self, url, callback=None):
        seq, data = self.encode_request('GET', url)
        self.session.send_encrypted_packet(0xb2, data)
        if callback:
            self.callbacks[seq] = callback

    def encode_request(self, method, url, mime=None, payload=None):
        request = protocol.MercuryRequest()
        request.url = url
        request.method = method
        if mime:
            request.mime = mime

        seq = struct.pack('>L', self.seq)
        self.seq += 1

        count = 2 if payload else 1

        header = struct.pack('>H', len(seq)) + \
                seq + \
                chr(1) + \
                struct.pack('>H', count)

        data = header + struct.pack('>H', request.ByteSize()) + request.SerializeToString()
        if payload:
            data += struct.pack('>H', payload.ByteSize()) + payload.SerializeToString()

        return seq, data

    def handle_packet(self, data):
        seq, flags, count, data = self.parse_header(data)

        offset = 0
        frames = []
        for i in range(count):
            length, = struct.unpack_from('>H', data, offset)
            offset += 2
            frames.append(data[offset:offset+length])
            offset += length

        response = protobuf_parse(protocol.MercuryReply, frames[0])
        if len(frames) > 1:
            payload = decode_payload(frames[1], response.mime)
        else:
            payload = None

        callback = self.callbacks.get(seq)
        if callback:
            more = callback(response, payload)
            if not more:
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

