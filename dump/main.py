from __future__ import absolute_import, division, print_function

import hashlib
import hmac
import os
import sys
import subprocess
import struct
import traceback
from hexdump import dump as hexdump

sys.path.append('../common')
import pylibrespot
from pylibrespot import mercury, protocol, packet, util
from pylibrespot.crypto import bin2bn, Crypto
import pyshn as shn

if len(sys.argv) < 4:
    print('Usage: %s KEYFILE USERNAME PASSWORD' % sys.argv[0], file=sys.stderr)
    sys.exit(1)

KEYFILE = sys.argv[1]
USERNAME = sys.argv[2]
PASSWORD = sys.argv[3]

class Reader:
    def __init__(self, fd):
        self.fd = fd
        self.buffer = [str(), str(), str()]

    def reader(self, type):
        def read(n):
            return self.read(type, n)
        return read

    def read(self, type, n):
        while len(self.buffer[type]) < n:
            self.recv()
        data = self.buffer[type][:n]
        self.buffer[type] = self.buffer[type][n:]
        return data

    def _recv(self, n):
        return os.read(self.fd, n)

    def recv(self):
        t,l = struct.unpack('>BL', self._recv(5))
        data = self._recv(l)
        self.buffer[t] += data

    def available(self, type):
        return len(self.buffer[type]) > 0

try:
    data_pipe = os.pipe()
    p = subprocess.Popen([
        './main',
        KEYFILE,
        USERNAME,
        PASSWORD,
        '/proc/self/fd/%d' % data_pipe[1]])
    
    reader = Reader(data_pipe[0])
    crypto = Crypto()

    prefix, header, data = packet.recv_packet(reader.reader(2), 2)
    init_client_packet = prefix + header + data

    private_key = reader.read(0, 0x60)
    crypto.generate_keys(private_key)

    r = protocol.Request()
    r.ParseFromString(data)
    assert r.data2.data0.data0 == crypto.public_key

    header, data = packet.recv_packet(reader.reader(1))
    init_server_packet = header + data

    r = protocol.Response()
    r.ParseFromString(init_server_packet[4:])

    crypto.compute_shared_key(r.data.data0.data0.data0)
    crypto.compute_challenge(init_client_packet, init_server_packet)

    header, data = packet.recv_packet(reader.reader(2))
    r = protocol.ChallengePacket()
    r.ParseFromString(data)
    assert r.data0.data0.data0 == crypto.challenge

    while True:
        reader.recv()

        if reader.available(1):
            cmd, data = packet.recv_encrypted_packet(
                    reader.reader(1),
                    crypto.recv_cipher)
            print('recv cmd: %x %x' % (cmd, len(data)))
        if reader.available(2):
            cmd, data = packet.recv_encrypted_packet(
                    reader.reader(2),
                    crypto.send_cipher)
            print('send cmd: %x %x' % (cmd, len(data)))
            print(data)
finally:
    p.kill()
    p.wait()

