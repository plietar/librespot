from __future__ import absolute_import, division, print_function

import os
import socket
import struct
import hashlib
import sys
from hexdump import dump as hexdump

sys.path.append('../common')
import pyshn as shn
from pyspotify import Session, Commands, protocol, protobuf_parse
from handlers import Handlers

ap = ("lon3-accesspoint-a26.ap.spotify.com", 4070)
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(ap)

if len(sys.argv) < 4:
    print("Usage: %s KEYFILE USERNAME PASSWORD" % sys.argv[0], file=sys.stderr)
    sys.exit(1)

KEYFILE = sys.argv[1]
USERNAME = sys.argv[2]
PASSWORD = sys.argv[3]

with open(KEYFILE, 'r') as f:
    session = Session(sock, f.read(), {
        'brand': 'DemoBrand',
        'model': 'DemoModel',
        'serial': hashlib.sha1('%s' % os.getpid()).hexdigest() })
commands = Commands(session)
handlers = Handlers(session, commands)

session.crypto.generate_keys()

r = protocol.Request()

r.data0.data0 = 0x05;
r.data0.data1 = 0x02;
r.data0.data2 = 0x10800000000;
r.data1 = 0
r.data2.data0.data0 = session.crypto.public_key;
r.data2.data0.data1 = 1
r.random = os.urandom(0x10)
r.data4 = str(bytearray([0x1e]))
r.data5 = str(bytearray([0x08, 0x01]))

init_client_packet = session.send_packet(
        r.SerializeToString(), extra=struct.pack('>H', 4))

header, data = session.recv_packet()
init_server_packet = header + data

r = protobuf_parse(protocol.Response, data)

session.crypto.compute_shared_key(r.data.data0.data0.data0)
session.crypto.compute_challenge(init_client_packet, init_server_packet)

r = protocol.ChallengePacket()
r.data0.data0.data0 = session.crypto.challenge
r.data1 = ''
r.data2 = ''

session.send_packet(r.SerializeToString())

commands.authenticate(USERNAME, PASSWORD)

while True:
    cmd, data = session.recv_encrypted_packet()
    handlers(cmd, data)

s.close()

