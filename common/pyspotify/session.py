from __future__ import absolute_import, division, print_function, unicode_literals

from . import packet
from .crypto import Crypto

import os
import struct
import pyshn as shn

class Session(object):
    def __init__(self, sock, appkey, device):
        self.sock = sock
        self.crypto = Crypto()
        self.appkey = appkey
        self.device = device

    def send_raw(self, data):
        self.sock.sendall(data)

    def recv_raw(self, length):
        data = bytes()
        while len(data) < length:
            d = self.sock.recv(length - len(data))
            if d is None:
                raise Exception('socket closed')
            data += d
        assert len(data) == length, \
                '%x < %x' % (len(data), length)
        return data

    def send_packet(self, data, extra=bytes()):
        return packet.send_packet(self.send_raw, data, extra)

    def recv_packet(self):
        return packet.recv_packet(self.recv_raw)

    def send_encrypted_packet(self, cmd, data):
        return packet.send_encrypted_packet(self.send_raw,
                self.crypto.send_cipher, cmd, data)

    def recv_encrypted_packet(self):
        return packet.recv_encrypted_packet(self.recv_raw,
                self.crypto.recv_cipher)

