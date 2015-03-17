cdef extern from "shn.h":
    ctypedef struct shn_ctx:
        pass
    void shn_key (shn_ctx * c, unsigned char key[], int keylen)
    void shn_nonce (shn_ctx * c, unsigned char nonce[], int nlen)
    void shn_stream (shn_ctx * c, unsigned char *buf, int nbytes)
    void shn_maconly (shn_ctx * c, unsigned char *buf, int nbytes)
    void shn_encrypt (shn_ctx * c, unsigned char *buf, int nbytes)
    void shn_decrypt (shn_ctx * c, unsigned char *buf, int nbytes)
    void shn_finish (shn_ctx * c, unsigned char *buf, int nbytes)

