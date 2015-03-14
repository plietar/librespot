#ifndef CALLBACKS_H
#define CALLBACKS_H

#include "spotify.h"

extern struct connection_callbacks connection_callbacks;
extern struct debug_callbacks debug_callbacks;
extern struct playback_callbacks playback_callbacks;
void error_callback(sp_err_t err);

#endif

