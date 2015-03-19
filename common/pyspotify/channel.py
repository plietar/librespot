from __future__ import absolute_import, division, print_function

from enum import Enum
import struct

class Channel(object):
    def __init__(self, callback):
        self.dataMode = False
        self.callback = callback
        self.header_id = 0

class ChannelManager(object):
    def __init__(self):
        self.channels = dict()
        self.counter = 0

    def create(self, callback):
        n = self.counter
        self.channels[n] = Channel(callback)
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
                    # Do something with header
                    ch.header_id += 1
                    offset += length
                if length == 0:
                    ch.dataMode = True
            else:
                if len(data) == 0:
                    ch.callback(number, None)
                    del self.channels[number]
                else:
                    ch.callback(number, data)

