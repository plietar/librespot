from libc.stdint cimport int64_t

cdef extern from "vorbis/vorbisfile.h":
    ctypedef struct OggVorbis_File:
        pass
    ctypedef struct ov_callbacks:
        size_t (*read_func) (void *ptr, size_t size, size_t nmemb, void *datasource);
        int (*seek_func) (void *datasource, long offset, int whence);
        int (*close_func) (void *datasource);
        long (*tell_func) (void *datasource);
    ctypedef struct vorbis_info:
        int version
        int channels
        long rate

        long bitrate_upper
        long bitrate_nominal
        long bitrate_lower
        long bitrate_window

    int ov_open_callbacks(void *datasource,
            OggVorbis_File *vf,
            char *initial, long ibytes,
            ov_callbacks callbacks)
    
    double ov_time_total(OggVorbis_File *vf, int i)
    long ov_streams(OggVorbis_File *vf)
    long ov_read(OggVorbis_File *vf, char *buffer, int length, int bigendianp, int word, int sgned, int *bitstream)
    int ov_pcm_seek(OggVorbis_File *vf, int64_t pos)
    int ov_time_seek(OggVorbis_File *vf, double s)

    vorbis_info *ov_info(OggVorbis_File *vf,int link)
    

    #ctypedef struct shn_ctx:
    #    pass
    #void shn_key (shn_ctx * c, unsigned char key[], int keylen)
    #void shn_nonce (shn_ctx * c, unsigned char nonce[], int nlen)
    #void shn_stream (shn_ctx * c, unsigned char *buf, int nbytes)
    #void shn_maconly (shn_ctx * c, unsigned char *buf, int nbytes)
    #void shn_encrypt (shn_ctx * c, unsigned char *buf, int nbytes)
    #void shn_decrypt (shn_ctx * c, unsigned char *buf, int nbytes)
    #void shn_finish (shn_ctx * c, unsigned char *buf, int nbytes)

