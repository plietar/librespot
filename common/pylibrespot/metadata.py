from __future__ import absolute_import, division, print_function

from .util import uri2gid, gid2id

def loaded(f):
    def inner(self, *args, **kwargs):
        if not self.is_loaded:
            return None
        else:
            return f(self, *args, **kwargs)
    return inner

class MetadataObject(object):
    def __init__(self, data=None, uri=None):
        if data is not None:
            self._is_loaded = True
            self._data = data
            self._gid = data.gid
        elif uri is not None:
            self._is_loaded = False
            self._data = None
            self._gid = uri2gid(uri)
        else:
            raise ValueError("either data or uri must be given")

    def load_callback(self, header, data):
        self._data = data
        self._is_loaded = True

    @property
    def is_loaded(self):
        return self._is_loaded

    @property
    def gid(self):
        return self._gid

    @property
    def id(self):
        return gid2id(self._gid)

class Track(MetadataObject):
    @property
    @loaded
    def name(self):
        return self._data.name

    @property
    @loaded
    def duration(self):
        return self._data.duration

    @property
    @loaded
    def file(self):
        return File(data=self._data.file[0])

class File(MetadataObject):
    pass

