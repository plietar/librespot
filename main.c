#include "spotify.h"
#include "audio.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <time.h>

#ifndef USERNAME
#define USERNAME "user"
#endif

#ifndef PASSWORD
#define PASSWORD "password"
#endif

void error_callback(sp_err_t err) {
    printf("error: %d\n", err);
}

void connection_notify(sp_connection_notify_t type, void *userdata) {
    switch(type) {
        case kSpConnectionNotifyLoggedIn:
            printf("kSpConnectionNotifyLoggedIn\n");
            break;
        default:
            printf("UNKNOWN ConnectionNotify %d\n", type);
    }
}

void connection_message(char *msg, void *userdata) {
    printf("connection_message: %s, %p\n", msg, userdata);
}

void debug_message(char *msg, void *userdata) {
    printf("debug_message: %s\n", msg);
}

void playback_notify(sp_playback_notify_t type, void *userdata) {
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

void playback_data(void *frames, uint32_t num_frames, sp_audioformat *format, void *userdata) {
    audio_frame(frames, num_frames, format);
}

void playback_seek(uint32_t millis, void *userdata) {
    printf("playback_seek: %d\n", millis);
}

void playback_volume(uint16_t volume, void *userdata) {
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

int read_app_key(char *path, uint8_t *data, int size) {
    int fd = open(path, O_RDONLY);
    size = read(fd, data, size);
    close(fd);
    return size;
}

int print_vars(void) {
    struct vars_data vars;
    int ret;

    memset(&vars, 0x0, sizeof(vars));
    ret = SpZeroConfGetVars(&vars);

    printf("%s\n", vars.publicKey);
    printf("%s\n", vars.deviceId);
    printf("%s\n", vars.remoteName);
    printf("%s\n", vars.accountReq);
    printf("%s\n", vars.deviceType);

    return ret;
}

int main(void) {
    struct init_data init;
    char os_device_id[0x20];

    memset(&init, 0x0, sizeof(init));

    snprintf(os_device_id, 0x20, "ee:27:40:d9:35:a7-%d", getpid());

    init.version = 4;
    init.buffer = malloc(0x100000);
    init.buffer_size = 0x100000;
    init.app_key = malloc(0x190);
    init.app_key_size = read_app_key("spotify_appkey.key", init.app_key, 0x190);
    init.os_device_id = os_device_id;
    init.remoteName = "TestConnect";
    init.brandName = "DummyBrand";
    init.modelName = "DummyModel";
    init.deviceType = kSpDeviceTypeComputer;
    init.error_callback = error_callback;
    init.zero1 = 0;

    printf("Buffer: %p\n", init.buffer);

    SpInit(&init);
    SpRegisterPlaybackCallbacks(&playback_callbacks, NULL);
    SpRegisterConnectionCallbacks(&connection_callbacks, NULL);
    SpRegisterDebugCallbacks(&debug_callbacks, NULL);
    SpPlaybackUpdateVolume(0x8000);
    SpConnectionLoginPassword(USERNAME, PASSWORD);

    audio_init();

    struct timespec sleep;
    while (1) {
        SpPumpEvents();

        sleep.tv_sec = 0;
        sleep.tv_nsec = 1000;
        nanosleep(&sleep, NULL);
    }

    audio_close();

    return 0;
}

