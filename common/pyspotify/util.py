from __future__ import absolute_import, division, print_function

import struct
from google.protobuf.descriptor import FieldDescriptor

from . import protocol


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

