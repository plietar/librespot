from __future__ import absolute_import, division, print_function

import alsaaudio

PERIODSIZE = 44100

class Sink(object):
    def start(self):
        pass
    def stop(self):
        pass
    def write(self, frames):
        pass
    def flush(self):
        pass

class AlsaSink(Sink):
    def __init__(self, card='default'):
        self.device = alsaaudio.PCM(
                alsaaudio.PCM_PLAYBACK,
                card = card)
        self.device.setperiodsize(PERIODSIZE)

    def start(self):
        pass
    def stop(self):
        pass
    def flush(self):
        pass

    def write(self, data):
        self.device.write(data)


