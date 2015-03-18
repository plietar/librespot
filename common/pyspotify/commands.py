from __future__ import absolute_import, division, print_function

from . import protocol
from . import mercury

class Commands:
    def __init__(self, session):
        self.session = session

    def authenticate(self, username, password):
        packet = protocol.AuthRequest()
        packet.data0.username = username
        packet.data0.data1 = 0
        packet.data0.password = password

        packet.data1.data0 = 0
        packet.data1.data1 = 0
        packet.data1.partner = 'Partner lenbrook_bluesound %s;%s' % (
                self.session.device['brand'],
                self.session.device['model'])
        packet.data1.deviceid = self.session.device['serial']

        packet.version = 'master-v1.8.0-gad9e5b46'

        packet.data3.data0 = 1
        packet.data3.appkey1 = self.session.appkey[0x1:0x81]
        packet.data3.appkey2 = self.session.appkey[0x81:0x141]
        packet.data3.data3 = ''
        packet.data3.data4 = '\0' * 20

        self.session.send_encrypted_packet(0xab, packet.SerializeToString())

    def pong(self, token):
        self.session.send_encrypted_packet(0x49, token)

    def mercury_subscribe(self, url):
        data = mercury.encode_request('SUB', url)
        self.session.send_encrypted_packet(0xb3, data)

    def mercury_multiget(self, url, urls):
        payload = protocol.MercuryMultiGetRequest()
        for u in urls:
            p = payload.request.add()
            p.url = u
        data = mercury.encode_request('GET', url,
                mime='vnd.spotify/mercury-mget-request',
                payload=payload)
        self.session.send_encrypted_packet(0xb2, data)

