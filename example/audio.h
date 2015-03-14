#ifndef AUDIO_H
#define AUDIO_h

#include "spotify.h"

int audio_init(void);
void audio_frame(const void *frames, uint32_t num_frames, sp_audioformat *format);
void audio_flush(void);
void audio_volume(uint16_t volume);
void audio_close(void);

#endif

