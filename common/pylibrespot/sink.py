from __future__ import absolute_import, division, print_function

import alsaaudio

class Sink(object):
    def start(self):
        pass
    def stop(self):
        pass
    def write(self, frames):
        pass
    def flush(self):
        pass

FRAME_SIZE=4
PERIOD_SIZE=44100 // 4# 0.25s
BUFFER_SIZE=PERIOD_SIZE * FRAME_SIZE

class AlsaSink(Sink):
    def __init__(self, card='default'):
        self.device = alsaaudio.PCM(
                alsaaudio.PCM_PLAYBACK,
                card = card)
        self.device.setperiodsize(PERIOD_SIZE)
        self.device.setchannels(2)
        self.device.setrate(44100)
        self.device.setformat(alsaaudio.PCM_FORMAT_S16_LE)

        self.buffer = str()

    def start(self):
        pass
    def stop(self):
        pass
    def flush(self):
        pass

    def write(self, data):
        self.buffer += data
        while len(self.buffer) > BUFFER_SIZE:
            ret = self.device.write(self.buffer[:BUFFER_SIZE])
            self.buffer = self.buffer[BUFFER_SIZE:]


