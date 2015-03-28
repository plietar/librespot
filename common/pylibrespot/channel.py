from __future__ import absolute_import, division, print_function

from enum import Enum
import struct

class Channel(object):
    def __init__(self, on_header, on_data):
        self.dataMode = False
        self.on_header = on_header
        self.on_data = on_data

class ChannelManager(object):
    def __init__(self):
        self.channels = dict()
        self.counter = 0

    def create(self, on_header = None, on_data = None):
        n = self.counter
        self.channels[n] = Channel(on_header, on_data)
        self.counter += 1
        return n

    def handle_packet(self, number, data):
        if number in self.channels:
            ch = self.channels[number]
            if not ch.dataMode:
                offset = 0
                length = 0
                while offset < len(data):
                    length, = struct.unpack_from('>H', data, offset)
                    offset += 2
                    if length > 0:
                        header_id, = struct.unpack_from('>b', data, offset)
                        if ch.on_header:
                            ch.on_header(number, header_id, data[offset+1:offset+length])
                    offset += length
                if length == 0:
                    if ch.on_data:
                        ch.dataMode = True
                    else:
                        del self.channels[number]
            else:
                if len(data) == 0:
                    ch.on_data(number, None)
                    del self.channels[number]
                else:
                    ch.on_data(number, data)

