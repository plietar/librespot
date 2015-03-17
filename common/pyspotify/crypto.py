from __future__ import absolute_import, division, print_function, unicode_literals

import os

import pyshn as shn
import hashlib
import hmac

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
        data += b'\0' * (length - len(data))
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

class Crypto(object):
    def __init__(self):
        self.private_key = None
        self.public_key = None
        self.remote_key = None
        self.shared_key = None
        self.send_key = None
        self.send_cipher = None
        self.recv_key = None
        self.recv_cipher = None
        self.challenge = None

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

        self.recv_key = data[0x34:0x54]
        self.recv_cipher = shn.Shannon(self.recv_key)

