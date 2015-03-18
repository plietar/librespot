from __future__ import absolute_import, division, print_function

import struct
from hexdump import dump as hexdump

from .util import protobuf_parse
from . import protocol

PROTOCOLS = {
        'vnd.spotify/mercury-mget-request': protocol.MercuryMultiGetRequest,
        'vnd.spotify/mercury-mget-reply': protocol.MercuryMultiGetReply,
}

def find_protocol(type):
    return PROTOCOLS[type] if type else None

def decode_payload(data, type=None, schema=None):
    proto = find_protocol(type) if type else schema
    if proto:
        return protobuf_parse(proto, data)
    else:
        return None

def encode_request(method, url, mime = None, payload=None, seq=b'\0\0\0\0'):
    request = protocol.MercuryRequest()
    request.url = url
    request.method = method
    if mime:
        request.mime = mime

    count = 2 if payload else 1

    header = struct.pack('>H', len(seq)) + \
            seq + \
            chr(1) + \
            struct.pack('>H', count)

    data = header + struct.pack('>H', request.ByteSize()) + request.SerializeToString()
    if payload:
        data += struct.pack('>H', payload.ByteSize()) + payload.SerializeToString()
    return data

continuation_frames = {}
continuation_data = {}

def parse_reply(cmd, data, schema=None, t=protocol.MercuryReply):
    global continuation_data
    global continuation_frames

    offset = 0
    seq_length, = struct.unpack_from('>H', data, offset)
    offset += 2
    seq = data[offset:offset+seq_length]
    offset += seq_length
    flags = ord(data[offset])
    offset += 1
    count, = struct.unpack_from('>H', data, offset)
    offset += 2

    print('seq="%s" flags=%d count=%d' % (hexdump(seq), flags, count))

    if flags & 0x2:
        continuation_data.setdefault(seq, '')

        for i in range(count):
            length, = struct.unpack_from('>H', data, offset)
            offset += 2
            continuation_data[seq] += data[offset:offset+length]
            offset += length
        return seq, None, None

    frames = []
    for i in range(count):
        length, = struct.unpack_from('>H', data, offset)
        offset += 2
        p = continuation_data.get(seq, '') + data[offset:offset+length]
        frames.append(p)
        continuation_data[seq] = ''
        
        offset += length

    if seq in continuation_frames:
        continuation_frames[seq] += frames
    else:
        continuation_frames[seq] = frames

    if flags & 0x1:
        frames = continuation_frames[seq]
        continuation_frames[seq] = []
        assert len(frames) > 0

        reply = protobuf_parse(t, frames[0])
        payloads = []

        for f in frames[1:]:
            payload = decode_payload(f, type=reply.mime, schema=schema)
            payloads.append(payload)
        return seq, reply, payloads
    else:
        return seq, None, None

