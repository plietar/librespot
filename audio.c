#include "audio.h"
#include "spotify.h"
#include <ao/ao.h>
#include <stdio.h>
#include <unistd.h>
#include <signal.h>
#include <sys/wait.h>

/*
 * qemu-arm does not support alsa.
 * Fork to a native process and stream data through stdin
 */

static int audio_fd = -1;
static int audio_pid = -1;

int audio_init(void) {
    int fds[2];
    pipe(fds);

    audio_pid = fork();
    if (audio_pid == 0) {
        dup2(fds[0], 0);
        close (fds[0]);
        close (fds[1]);
        execl("/usr/bin/aplay", "aplay",
                "-t", "raw",
                "-r", "44100",
                "-f", "S16_LE",
                "-c", "2",
                "-", NULL);
        perror("exec");
        exit(1);
    } else {
        close (fds[0]);
        audio_fd = fds[1];
    }

    return 0;
}

void audio_frame(void *frames, uint32_t num_frames, sp_audioformat *format) {
    if (format->sample_type != SP_SAMPLETYPE_INT16_NATIVE_ENDIAN
            || format->sample_rate != 44100
            || format->channels != 2) {
        printf("Wrong audio format: %d %d %d\n", format->sample_type, format->sample_rate, format->channels);
    } else {
        write(audio_fd, frames, num_frames * 2);
    }
}

void audio_flush(void) {
}

void audio_volume(uint16_t volume) {
}

void audio_close(void) {
    kill(audio_pid, SIGKILL);
    waitpid(audio_pid, NULL, 0);
    close(audio_fd);
}

