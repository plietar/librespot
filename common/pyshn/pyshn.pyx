from pyshn cimport *
from libc.stdlib cimport malloc, free
from libc.string cimport memcpy
from libc.stdint cimport uint8_t
import struct

cdef class Shannon:
    cdef shn_ctx* _ctx
    cdef public int nonce

    def __cinit__(self, bytes k):
        self.nonce = 0

        self._ctx = <shn_ctx*>malloc(sizeof(shn_ctx))
        shn_key(self._ctx, k, len(k))

    def reset(self):
        shn_nonce(self._ctx, struct.pack('>L', self.nonce), 4)

    def decrypt(self, bytes k):
        cdef uint8_t *output = <uint8_t*>malloc(len(k))
        memcpy(output, <char*>k, len(k))
        shn_decrypt(self._ctx, output, len(k))
        return output[:len(k)]

    def encrypt(self, bytes k):
        cdef uint8_t *output = <uint8_t*>malloc(len(k))
        memcpy(output, <char*>k, len(k))
        shn_encrypt(self._ctx, output, len(k))
        return output[:len(k)]

    def finish(self, int size):
        cdef uint8_t *output = <uint8_t*>malloc(size)
        shn_finish(self._ctx, output, size)
        return output[:size]

    def __dealloc__(self):
        free(self._ctx)

