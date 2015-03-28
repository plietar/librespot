from vorbisfile cimport *
from libc.string cimport memcpy
from libc.stdlib cimport malloc
from cpython.string cimport PyString_AS_STRING, PyString_GET_SIZE, PyString_FromStringAndSize

cdef class Decoder:
    cdef OggVorbis_File vf
    cdef object f
    cdef int bitstream

    def __cinit__(self, f):
        self.f = f
        ret = ov_open_callbacks(<void *>self, &self.vf, NULL, 0, callbacks)
        self.bitstream = 0

        print('open: %d' % ret)

    def time_total(self, int i):
        return ov_time_total(&self.vf, i)

    def streams(self):
        return ov_streams(&self.vf)

    def read(self, length):
        cdef char *buffer = <char*>malloc(length)
        n = ov_read(&self.vf, buffer, length, 0, 2, 1, &self.bitstream)
        return PyString_FromStringAndSize(buffer, n)

    def pcm_seek(self, int64_t pos):
        return ov_pcm_seek(&self.vf, pos)

    def time_seek(self, double s):
        return ov_time_seek(&self.vf, s)

cdef size_t read_cb(void *ptr, size_t size, size_t nmemb, void *datasource):
    self = <Decoder>datasource
    data = self.f.read(size * nmemb)
    length = PyString_GET_SIZE(data)
    if length > size*nmemb:
        print('Warning size mismatch')
    length = min(length, size*nmemb)
    memcpy(ptr, PyString_AS_STRING(data), length)
    return length

cdef int seek_cb(void *datasource, long offset, int whence):
    self = <Decoder>datasource
    self.f.seek(offset, whence)

cdef int close_cb(void *datasource):
    self = <Decoder>datasource
    self.f.close()
    return 0

cdef long tell_cb(void *datasource):
    self = <Decoder>datasource
    return self.f.tell()


cdef ov_callbacks callbacks
callbacks.read_func = read_cb
callbacks.seek_func = seek_cb
callbacks.close_func = close_cb
callbacks.tell_func = tell_cb


