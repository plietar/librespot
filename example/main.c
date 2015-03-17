#include "spotify.h"
#include "audio.h"
#include "callbacks.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <time.h>

static int read_app_key(const char *path, uint8_t *data, int size) {
    int fd = open(path, O_RDONLY);
    size = read(fd, data, size);
    close(fd);
    return size;
}

static int print_vars(void) {
    struct vars_data vars;
    int ret;

    memset(&vars, 0x0, sizeof(vars));
    ret = SpZeroConfGetVars(&vars);

    printf("public key: %s\n", vars.publicKey);
    printf("device id: %s\n", vars.deviceId);
    printf("remote name: %s\n", vars.remoteName);
    printf("account req: %s\n", vars.accountReq);
    printf("device type: %s\n", vars.deviceType);

    return ret;
}

int main(int argc, const char *argv[]) {
    struct init_data init;
    char os_device_id[0x20];
    int debug_flag = 0;
    const char *keyfile;

    if (argc < 2) {
        fprintf(stderr, "Usage: %s [-d] KEYFILE\n", argv[0]);
        exit(1);
    }

    if (argc > 2 && strcmp("-d", argv[1]) == 0) {
        debug_flag = 1;
        appkey_file = argv[2];
    } else {
        appkey_file = argv[1];
    }

    memset(&init, 0x0, sizeof(init));

    snprintf(os_device_id, 0x20, "abcdef %d", getpid());

    init.version = 4;
    init.buffer = malloc(0x100000);
    init.buffer_size = 0x100000;
    init.app_key = malloc(0x190);
    init.app_key_size = read_app_key(keyfile, init.app_key, 0x190);
    init.os_device_id = os_device_id;
    init.remoteName = "TestConnect";
    init.brandName = "DummyBrand";
    init.modelName = "DummyModel";
    init.deviceType = kSpDeviceTypeComputer;
    init.error_callback = error_callback;
    init.zero1 = 0;

    SpInit(&init);

    SpRegisterPlaybackCallbacks(&playback_callbacks, NULL);
    SpRegisterConnectionCallbacks(&connection_callbacks, NULL);
    if (debug_flag) {
        SpRegisterDebugCallbacks(&debug_callbacks, NULL);
    }
    SpPlaybackUpdateVolume(0x8000);

    audio_init();

    print_vars();

    while (1) {
        SpPumpEvents();

        struct timeval tv;
        fd_set fds;
        tv.tv_sec = 0;
        tv.tv_usec = 0;
        FD_ZERO(&fds);
        FD_SET(STDIN_FILENO, &fds);
        select(STDIN_FILENO+1, &fds, NULL, NULL, &tv);

        if (FD_ISSET(0, &fds)) {
            char *line = NULL;
            size_t n = 0;

            getline(&line, &n, stdin);

            char *cmd = strtok(line, " \t\n");
            if (cmd) {
                if (strcmp(cmd, "play") == 0) {
                    SpPlaybackPlay();
                } else if (strcmp(cmd, "pause") == 0) {
                    SpPlaybackPause();
                } else if (strcmp(cmd, "prev") == 0) {
                    SpPlaybackSkipToPrev();
                } else if (strcmp(cmd, "next") == 0) {
                    SpPlaybackSkipToNext();
                } else if (strcmp(cmd, "info") == 0) {
                    printf("active: %d\n", SpPlaybackIsActiveDevice());
                    printf("playing: %d\n", SpPlaybackIsPlaying());
                    printf("shuffled: %d\n", SpPlaybackIsShuffled());
                    printf("repeated: %d\n", SpPlaybackIsRepeated());
                    printf("logged in: %d\n", SpConnectionIsLoggedIn());
                } else if (strcmp(cmd, "metadata") == 0) {
                    struct SpMetadata metadata;
                    SpGetMetadata(&metadata, 0);
                    printf("Currently playing :\n");
                    printf("\t%s\n", metadata.context_uri);

                    printf("\t%s (%s)\n", metadata.track_name, metadata.track_uri);
                    printf("\t%s (%s)\n", metadata.artist_name, metadata.artist_uri);
                    printf("\t%s (%s)\n", metadata.album_name, metadata.album_uri);
                    int duration = metadata.duration / 1000;
                    printf("\t%d:%02d\n", duration / 60, duration % 60);

                    char url[512];
                    SpGetMetadataImageURL(metadata.cover_uri, kSpImageSize640, url, sizeof(url));
                    printf("\t%s\n", url);
                } else if (strcmp(cmd, "login") == 0) {
                    char *username = strtok(NULL, " \t\n");
                    char *password = strtok(NULL, " \t\n");
                    printf("%s %s\n", username, password);
                    SpConnectionLoginPassword(username, password);
                } else if (strcmp(cmd, "logout") == 0) {
                    SpConnectionLogout();
                } else if (strcmp(cmd, "quit") == 0) {
                    break;
                }
            }
        }
    }

    SpFree();

    audio_close();

    return 0;
}

