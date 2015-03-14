#include "spotify.h"
#include "audio.h"
#include "callbacks.h"

#include <stdio.h>

void error_callback(sp_err_t err) {
    printf("error: %d\n", err);
}

static void connection_notify(sp_connection_notify_t type, void *userdata) {
    switch(type) {
        case kSpConnectionNotifyLoggedIn:
            printf("kSpConnectionNotifyLoggedIn\n");
            break;
        case kSpConnectionNotifyLoggedOut:
            printf("kSpConnectionNotifyLoggedOut\n");
        default:
            printf("UNKNOWN ConnectionNotify %d\n", type);
    }
}

static void connection_message(const char *msg, void *userdata) {
    printf("connection_message: %s\n", msg);
}

static void debug_message(const char *msg, void *userdata) {
    printf("debug_message: %s\n", msg);
}

static void playback_notify(sp_playback_notify_t type, void *userdata) {
    switch(type) {
        case kSpPlaybackNotifyPause:
            printf("kSpPlaybackNotifyPause\n");
            break;
        case kSpPlaybackNotifyTrackChanged:
            printf("kSpPlaybackNotifyTrackChanged\n");
            break;
        case kSpPlaybackNotifyPlay:
            printf("kSpPlaybackNotifyPlay\n");
            break;
        case kSpPlaybackNotifyBecameActive:
            printf("kSpPlaybackNotifyBecameActive\n");
            break;
        case kSpPlaybackNotifyBecameInactive:
            printf("kSpPlaybackNotifyBecameInactive\n");
            break;
        case kSpPlaybackEventAudioFlush:
            printf("kSpPlaybackEventAudioFlush\n");
            audio_flush();
            break;
        default:
            printf("UNKNOWN PlaybackNotify %d\n", type);
    }
}

static void playback_data(const void *frames, uint32_t num_frames,
        sp_audioformat *format, void *userdata) {
    audio_frame(frames, num_frames, format);
}

static void playback_seek(uint32_t millis, void *userdata) {
    printf("playback_seek: %d\n", millis);
}

static void playback_volume(uint16_t volume, void *userdata) {
    printf("playback_volume: %d\n", volume);
    audio_volume(volume);
}

struct connection_callbacks connection_callbacks = {
    connection_notify,
    connection_message,
};

struct debug_callbacks debug_callbacks = {
    debug_message,
};

struct playback_callbacks playback_callbacks = {
    playback_notify,
    playback_data,
    playback_seek,
    playback_volume,
};

