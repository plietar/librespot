from __future__ import absolute_import, division, print_function, unicode_literals

import struct
from google.protobuf.descriptor import FieldDescriptor

from . import protocol

def bin2bn(data):
    x = 0
    n = len(data)
    for i, d in enumerate(data):
        x += ord(d) * 256**(n-i-1)
    return x

def bn2bin(bn, length=None):
    data = str()
    while bn > 0:
        data += chr(bn & 0xFF)
        bn = bn >> 8
    if len(data) < length:
        data += '\0' * (length - len(data))
    return data[::-1]

DH_generator = bin2bn(str(bytearray([ 0x02 ])))
DH_prime = bin2bn(str(bytearray([
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc9,
        0x0f, 0xda, 0xa2, 0x21, 0x68, 0xc2, 0x34, 0xc4, 0xc6,
        0x62, 0x8b, 0x80, 0xdc, 0x1c, 0xd1, 0x29, 0x02, 0x4e,
        0x08, 0x8a, 0x67, 0xcc, 0x74, 0x02, 0x0b, 0xbe, 0xa6,
        0x3b, 0x13, 0x9b, 0x22, 0x51, 0x4a, 0x08, 0x79, 0x8e,
        0x34, 0x04, 0xdd, 0xef, 0x95, 0x19, 0xb3, 0xcd, 0x3a,
        0x43, 0x1b, 0x30, 0x2b, 0x0a, 0x6d, 0xf2, 0x5f, 0x14,
        0x37, 0x4f, 0xe1, 0x35, 0x6d, 0x6d, 0x51, 0xc2, 0x45,
        0xe4, 0x85, 0xb5, 0x76, 0x62, 0x5e, 0x7e, 0xc6, 0xf4,
        0x4c, 0x42, 0xe9, 0xa6, 0x3a, 0x36, 0x20, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff ])))

def protobuf_parse(type, data):
    def check(r):
        for t, value in r._unknown_fields:
            tag = 0
            m = 1
            while ord(t[0]) > 0x80:
                tag += ord(t[0]) & 0x7f * m
                m *= 128
                t = t[1:]
            tag += ord(t[0]) * m
            print('WARNING unknown field: %x %x (%s)' % (tag >> 3, tag & 3, r.__class__.DESCRIPTOR.full_name))

        for desc, f in r.ListFields():
            if desc.type == FieldDescriptor.TYPE_MESSAGE:
                if desc.label != FieldDescriptor.LABEL_REPEATED:
                    f = [f]
                for g in f:
                    check(g)

        for desc in r.__class__.DESCRIPTOR.fields:
            if desc.label == FieldDescriptor.LABEL_REQUIRED and not r.HasField(desc.name):
                print('WARNING missing required field: %s' % desc.full_name)



    r = type()
    r.ParseFromString(data)
    check(r)

    return r

