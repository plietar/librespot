
class MetadataObject(object):
    def __init__(self):
        self._is_loaded = False

    @property
    def is_loaded(self):
        return self._is_loaded

    def load_callback(self, header, data):
        self._data = data
        self._is_loaded = True

class Track(MetadataObject):
    @property
    def name(self):
        if not self.is_loaded:
            return None
        else:
            return self._data.name

    @property
    def duration(self):
        if not self.is_loaded:
            return None
        else:
            return self._data.duration

