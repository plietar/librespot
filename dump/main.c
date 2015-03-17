#include <dlfcn.h>
#include <fcntl.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h> 
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

#include "spotify.h"

#define BUFFER_SIZE 0x100000

static void noop() {}

void debug_message(const char *msg, void *userdata) {
    printf("debug_message: %s\n", msg);
}

void error_callback(sp_err_t err) {
    fprintf(stderr, "Error occured: %d\n", err);
    exit(1);
}

struct connection_callbacks connection_callbacks = {
    noop,
    noop,
};

struct debug_callbacks debug_callbacks = {
    debug_message,
};

struct playback_callbacks playback_callbacks = {
    noop,
    noop,
    noop,
    (void *)noop,
};

static int read_app_key(const char *path, uint8_t *data, int size) {
    int fd = open(path, O_RDONLY);
    size = read(fd, data, size);
    close(fd);
    return size;
}

static int dump_fd = -1;
enum dump_type {
    DUMP_KEY = 0,
    DUMP_RECV = 1,
    DUMP_SEND = 2
};

static void dump_data(uint8_t type, const void *data, uint32_t size) {
    uint8_t header[5];
    header[0] = type;
    *((uint32_t*)&header[1]) = htonl(size);
    write(dump_fd, header, 5);
    write(dump_fd, data, size);
}

int main(int argc, const char *argv[]) {
    struct init_data init;
    char os_device_id[0x20];
    void *buffer = malloc(BUFFER_SIZE);

    int debug_flag = 0;
    const char *keyfile, *dumpfile;
    const char *username, *password;

    snprintf(os_device_id, 0x20, "%d", getpid());

    memset(&init, 0x0, sizeof(init));

    if (argc < 5) {
        fprintf(stderr, "Usage: %s [-d] KEYFILE USERNAME PASSWORD DUMPFILE\n",
                argv[0]);
        exit(1);
    }

    if (argc > 5 && strcmp("-d", argv[1]) == 0) {
        debug_flag = 1;
        keyfile = argv[2];
        username = argv[3];
        password = argv[4];
        dumpfile = argv[5];
    } else {
        keyfile = argv[1];
        username = argv[2];
        password = argv[3];
        dumpfile = argv[4];
    }

    dump_fd = creat(dumpfile, 0600);

    init.version = 4;
    init.buffer = buffer;
    init.buffer_size = BUFFER_SIZE;
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
    SpConnectionLoginPassword(username, password);

    const uint8_t *dh_ready = buffer + 0xbf888 + 0x25f;
    const uint8_t *dh_private = buffer + 0xbf888 + 0x261;
    bool key_dumped = false;

    struct timespec sleep;
    while (1) {
        SpPumpEvents();

        if (!key_dumped && *dh_ready) {
            dump_data(DUMP_KEY, dh_private, 0x60);
            key_dumped = true;
        }

        sleep.tv_sec = 0;
        sleep.tv_nsec = 1000;
        nanosleep(&sleep, NULL);
    }

    return 0;
}


static void *find_symbol(const char *name) {
    void *f = dlsym(RTLD_NEXT, name);
    if (!f) {
        fprintf(stderr, "Could not find symbol %s\n", name);
        exit(1);
    }
    return f;
}

static int ap_fd = -1;
static struct in_addr ap_addr;

int endswith(const char *str, const char *suffix) {
    size_t l1 = strlen(str);
    size_t l2 = strlen(suffix);

    if (l1 < l2) return 0;

    str += l1 - l2;

    return strcmp(str, suffix) == 0;
}

struct hostent *gethostbyname(const char *name) {
    static struct hostent *(*_gethostbyname)(const char*) = NULL;
    if (!_gethostbyname) _gethostbyname = find_symbol("gethostbyname");

    struct hostent *ret = _gethostbyname(name);

    if (ret && endswith(name, ".ap.spotify.com")) {
	memcpy (&ap_addr.s_addr, ret->h_addr, ret->h_length);
    }

    return ret;
}

int connect(int sockfd, const struct sockaddr *addr, socklen_t addrlen) {
    static int (*_connect)(int, const struct sockaddr *, socklen_t) = NULL;
    if (!_connect) _connect = find_symbol("connect");

    int ok = _connect(sockfd, addr, addrlen);

    if (addr->sa_family == AF_INET) {
        struct sockaddr_in *in_addr = (struct sockaddr_in *) addr;
        if (in_addr->sin_addr.s_addr == ap_addr.s_addr) {
            printf("connecting to ap on fd : %d\n", sockfd);
            ap_fd = sockfd;
        }
    }

    return ok;
}

ssize_t send(int s, const void *buf, size_t len, int flags) {
    static int (*_send)(int, const void *, size_t, int) = NULL;
    if (!_send) _send = find_symbol("send");

    int ret = _send(s, buf, len, flags);
    if (ret > 0 && s == ap_fd) {
        dump_data(DUMP_SEND, buf, ret);
    }

    return ret;
}

ssize_t recv(int s, void *buf, size_t len, int flags) {
    static int (*_recv)(int, void *, size_t, int) = NULL;
    if (!_recv) _recv = find_symbol("recv");

    int ret = _recv(s, buf, len, flags);
    if (ret > 0 && s == ap_fd) {
        dump_data(DUMP_RECV, buf, ret);
    }
    return ret;
}

int close(int fd) {
    static int (*_close)(int) = NULL;
    if (!_close) _close = find_symbol("close");

    if (fd == ap_fd) {
        ap_fd = -1;
    }

    return _close(fd);
}

