from __future__ import absolute_import, division, print_function

from . import protocol
from . import mercury
from .util import protobuf_parse

class Handlers(object):
    def __call__(self, cmd, data):
        if cmd == 0x4:
            self.ping(data)
        elif cmd == 0x4a:
            self.pongack(data)
        elif cmd == 0xac:
            packet = protobuf_parse(protocol.AuthSuccess, data)
            self.authentication_success(packet)
        elif cmd == 0xad:
            packet = protobuf_parse(protocol.AuthFailure, data)
            self.authentication_failure(packet)
        elif cmd == 0xb3:
            seq, response, payloads = mercury.parse_reply(cmd, data, schema=protocol.MercurySubscribed)
            if response:
                self.mercury(response, payloads)
        elif cmd == 0xb5:
            seq, response, payloads = mercury.parse_reply(cmd, data, schema=protocol.Frame)
            if response:
                self.mercury(response, payloads)
        elif cmd == 0xb2:
            seq, response, payloads = mercury.parse_reply(cmd, data)
            if response:
                self.mercury(response, payloads)

    def ping(self, token):
        pass
    def pongack(self, token):
        pass
    def authentication_success(self, packet):
        pass
    def authentication_failure(self, packet):
        pass
    def mercury(self, request, payloads):
        pass

