from __future__ import absolute_import, division, print_function

import os
import struct
from enum import Enum

from . import packet, protocol
from .crypto import Crypto
from .util import protobuf_parse
from .metadata import Track
from .mercury import Mercury
from .player import Player
from .channel import ChannelManager

class ConnectionState(Enum):
    LOGGED_OUT = 0
    LOGGED_IN = 1
    DISCONNECTED = 2
    UNDEFINED = 3
    OFFLINE = 4

class Session(object):
    def __init__(self, sock, appkey, device):
        self.sock = sock
        self.appkey = appkey
        self.device = device

        self.crypto = Crypto()
        self.crypto.generate_keys()

        self.player = Player(self)
        self.mercury = Mercury(self)

        self.channels = ChannelManager()

        self.connectionstate = ConnectionState.DISCONNECTED

    def connect(self):
        r = protocol.Request()

        r.data0.data0 = 0x05;
        r.data0.data1 = 0x02;
        r.data0.data2 = 0x10800000000;
        r.data1 = 0
        r.data2.data0.data0 = self.crypto.public_key;
        r.data2.data0.data1 = 1
        r.random = os.urandom(0x10)
        r.data4 = str(bytearray([0x1e]))
        r.data5 = str(bytearray([0x08, 0x01]))

        init_client_packet = self.send_packet(
                r.SerializeToString(), extra=struct.pack('>H', 4))

        header, data = self.recv_packet()
        init_server_packet = header + data

        r = protobuf_parse(protocol.Response, data)

        self.crypto.compute_shared_key(r.data.data0.data0.data0)
        self.crypto.compute_challenge(init_client_packet, init_server_packet)

        r = protocol.ChallengePacket()
        r.data0.data0.data0 = self.crypto.challenge
        r.data1 = ''
        r.data2 = ''
        self.send_packet(r.SerializeToString())

        self.connectionstate = ConnectionState.LOGGED_OUT

    def login(self, username, password):
        packet = protocol.AuthRequest()
        packet.data0.username = username
        packet.data0.data1 = 0
        packet.data0.password = password

        packet.data1.data0 = 0
        packet.data1.data1 = 0
        packet.data1.partner = 'Partner lenbrook_bluesound %s;%s' % (
                self.device['brand'],
                self.device['model'])
        packet.data1.deviceid = self.device['serial']

        packet.version = 'master-v1.8.0-gad9e5b46'

        packet.data3.data0 = 1
        packet.data3.appkey1 = self.appkey[0x1:0x81]
        packet.data3.appkey2 = self.appkey[0x81:0x141]
        packet.data3.data3 = ''
        packet.data3.data4 = '\0' * 20

        self.send_encrypted_packet(0xab, packet.SerializeToString())

    def poll(self):
        cmd, data = self.recv_encrypted_packet()
        self.handle(cmd, data)

    def handle(self, cmd, data):
        if cmd == 0xac:
            self.connectionstate = ConnectionState.LOGGED_IN
        elif cmd in range(0xb2, 0xb6):
            self.mercury.handle_packet(cmd, data)
        elif cmd == 0xd:
            id, key = struct.unpack('>L16s', data)
            self.player.handle_aeskey(id, key)
        elif cmd == 0xe:
            id, key = struct.unpack('>L16s', data)
            self.player.handle_aeskey(id, None)
        elif cmd == 0x9:
            id, = struct.unpack_from('>H', data, 0)
            self.channels.handle_packet(id, data[2:])

    def get_track(self, uri):
        t = Track(uri=uri)
        self.mercury.get(
                'hm://metadata/track/%s' % t.id,
                callback=t.load_callback)
        return t

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

