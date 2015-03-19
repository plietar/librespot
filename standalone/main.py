from __future__ import absolute_import, division, print_function

import os
import socket
import struct
import hashlib
import sys
from hexdump import dump as hexdump

sys.path.append('../common')
from pyspotify import Session, ConnectionState, PlayerState

ap = ("lon3-accesspoint-a26.ap.spotify.com", 4070)
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(ap)

if len(sys.argv) < 5:
    print("Usage: %s KEYFILE USERNAME PASSWORD TRACK" % sys.argv[0],
            file=sys.stderr)
    sys.exit(1)

KEYFILE = sys.argv[1]
USERNAME = sys.argv[2]
PASSWORD = sys.argv[3]
TRACK = sys.argv[4]

with open(KEYFILE, 'r') as f:
    session = Session(sock, f.read(), {
        'brand': 'DemoBrand',
        'model': 'DemoModel',
        'serial': hashlib.sha1('%s' % os.getpid()).hexdigest() })

session.connect()
session.login(USERNAME, PASSWORD)

while session.connectionstate != ConnectionState.LOGGED_IN:
    session.poll()

print('Logged in')

track = session.get_track(TRACK)

while not track.is_loaded:
    session.poll()

print('Loading "%s"' % track.name)

session.player.load(track)

while session.player.state != PlayerState.LOADED:
    session.poll()

print('Playing "%s"' % track.name)

session.player.play()

print('Done')

sock.close()

