from __future__ import absolute_import, division, print_function

import struct
from google.protobuf.descriptor import FieldDescriptor
import binascii

from . import protocol

def protobuf_parse(type, data):
    def check(r):
        if hasattr(r, '_unknown_fields'):
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

base62 = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
def gid2id(gid):
    return binascii.hexlify(gid).rjust(32, '0')

def id2uri(uritype, v):
    res = []
    v = int(v, 16)
    while v > 0:
        res = [v % 62] + res
        v //= 62
    id = ''.join([base62[i] for i in res])
    return ("spotify:"+uritype+":"+id.rjust(22, '0'))

def uri2id(uri):
    parts = uri.split(":")
    if len(parts) > 3 and parts[3] == "playlist":
        s = parts[4]
    else:
        s = parts[2]

    v = 0
    for c in s:
        v = v * 62 + base62.index(c)
    return hex(v)[2:-1].rjust(32, '0')

