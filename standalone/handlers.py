from __future__ import absolute_import, division, print_function

import pyspotify

class Handlers (pyspotify.Handlers):
    def __init__(self, session, commands):
        self.session = session
        self.commands = commands

    def __call__(self, cmd, data):
        super(Handlers, self).__call__(cmd, data)
        print('recv cmd %x %x' % (cmd, len(data)))
        
    def ping(self, token):
        self.commands.pong(token)

    def pongack(self, token):
        pass

    def authentication_success(self, packet):
        print(packet, end='')
        self.commands.mercury_subscribe(
                'hm://remote/user/%s/tinygaia/' % packet.username)

    def authentication_failure(self, packet):
        print(packet, end='')
        raise Exception('Authentication Failed')

    def mercury(self, request, payloads):
        print(request, end='')

