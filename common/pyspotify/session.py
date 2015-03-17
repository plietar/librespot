from __future__ import absolute_import, division, print_function, unicode_literals

from .util import bn2bin, bin2bn, DH_generator, DH_prime

import hashlib
import hmac
import os
import struct
import pyshn as shn

class Session:
    def __init__(self, sock, appkey, device):
        self.sock = sock

        self.private_key = None
        self.public_key = None
        self.remote_key = None
        self.shared_key = None
        self.send_key = None
        self.send_cipher = None
        self.send_nonce = 0
        self.recv_key = None
        self.recv_cipher = None
        self.recv_nonce = 0
        self.challenge = None
        self.appkey = appkey
        self.device = device

    def generate_keys(self, key = None):
        if key is not None:
            self.private_key = key
        else:
            self.private_key = b'\0' + os.urandom(0x5f)

        self.public_key = bn2bin(
                pow(DH_generator, bin2bn(self.private_key), DH_prime), 0x60)

    def compute_shared_key(self, remote):
        self.remote_key = remote
        self.shared_key = bn2bin(
                pow(bin2bn(remote), bin2bn(self.private_key), DH_prime), 0x60)

    def compute_challenge(self, client_packet, server_packet):
        data = bytes()
        for i in range(1,6):
            h = hmac.new(self.shared_key, digestmod=hashlib.sha1)
            h.update(client_packet)
            h.update(server_packet)
            if i is not None:
                h.update(chr(i))
            data += h.digest()

        h = hmac.new(data[:0x14], digestmod=hashlib.sha1)
        h.update(client_packet)
        h.update(server_packet)
        self.challenge = h.digest()

        self.send_key = data[0x14:0x34]
        self.send_cipher = shn.Shannon(self.send_key)
        self.send_nonce = 0

        self.recv_key = data[0x34:0x54]
        self.recv_cipher = shn.Shannon(self.recv_key)
        self.recv_nonce = 0

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

    def send_packet(self, packet, extra=bytes()):
        size = len(extra) + 4 + len(packet)
        data = extra + struct.pack('>L', size) + packet
        self.send_raw(data)

        return data

    def recv_packet(self):
        header = self.recv_raw(4)
        length, = struct.unpack('>L', header)
        data = self.recv_raw(length - 4)
        return header, data

    def send_encrypted_packet(self, cmd, packet):
        self.send_cipher.nonce(struct.pack('>L', self.send_nonce))
        header = struct.pack('>BH', cmd, len(packet))
        data = self.send_cipher.encrypt(header + packet)
        data += self.send_cipher.finish(4)
        self.send_raw(data)

        self.send_nonce += 1

    def recv_encrypted_packet(self):
        self.recv_cipher.nonce(struct.pack('>L', self.recv_nonce))

        header = self.recv_raw(3)
        header = self.recv_cipher.decrypt(header)

        cmd, length = struct.unpack('>BH', header)

        data = self.recv_raw(length)
        data = self.recv_cipher.decrypt(data)

        mac = self.recv_raw(4)
        assert self.recv_cipher.finish(4) == mac

        self.recv_nonce += 1
        return cmd, data

