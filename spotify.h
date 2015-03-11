#ifndef SPOTIFY_H
#define SPOTIFY_H

#include <stdint.h>

typedef enum {
    kSpErrorOk = 0,
    kSpErrorFailed = 1,
    kSpErrorInitFailed = 2,
    kSpErrorWrongAPIVersion = 3,
    kSpErrorNullArgument = 4,
    kSpErrorInvalidArgument = 5,
    kSpErrorUninitialized = 6,
    kSpErrorAlreadyInitialized = 7,
    kSpErrorLoginBadCredentials = 8,
    kSpErrorNeedsPremium = 9,
    kSpErrorTravelRestriction = 10,
    kSpErrorApplicationBanned = 11,
    kSpErrorGeneralLoginError = 12,
    kSpErrorUnsupported = 13,
    kSpErrorNotActiveDevice = 14,
    kSpErrorPlaybackErrorStart = 1000,
    kSpErrorGeneralPlaybackError = 1001,
    kSpErrorPlaybackRateLimited = 1002,
    kSpErrorUnknown = 1003,
} sp_err_t;

typedef enum {
    kSpConnectionNotifyLoggedIn = 0,
} sp_connection_notify_t;

typedef enum {
    kSpPlaybackNotifyPlay = 0,
    kSpPlaybackNotifyPause = 1,
    kSpPlaybackNotifyTrackChanged = 2,
    kSpPlaybackNotifyBecameActive = 9,
    kSpPlaybackNotifyBecameInactive = 10,
    kSpPlaybackEventAudioFlush = 12,
} sp_playback_notify_t;

typedef enum {
    kSpDeviceTypeUnknown = 0,
    kSpDeviceTypeComputer = 1,
    kSpDeviceTypeTablet = 2,
    kSpDeviceTypeSmartphone = 3,
    kSpDeviceTypeSpeaker = 4,
    kSpDeviceTypeTV = 5,
    kSpDeviceTypeAVR = 6,
    kSpDeviceTypeSTB = 7,
    kSpDeviceTypeAudioDongle = 8,
} sp_device_type_t;

typedef enum {
    SP_SAMPLETYPE_INT16_NATIVE_ENDIAN,
} sp_sampletype;

typedef struct {
    uint16_t channels;
    uint16_t sample_type;
    uint32_t sample_rate;
} sp_audioformat;

struct init_data {
    uint32_t version;
    uint8_t *buffer;
    uint32_t buffer_size; // 0x100000
    uint8_t *app_key;
    uint32_t app_key_size;
    char *os_device_id; // MAC-PID
    char *remoteName;
    char *brandName;
    char *modelName;
    uint32_t deviceType; // sp_device_type_t
    void (*error_callback)(sp_err_t err);
    uint32_t zero1;
};

struct vars_data {
    char publicKey[0x96];
    char activeUser[0x40];
    char deviceId[0x40];
    char remoteName[0x40];
    char accountReq[0x10];
    char deviceType[0x10];
};

struct connection_callbacks {
    void (*notify)(sp_connection_notify_t type, void *userdata);
    void (*message)(char *msg, void *userdata);
};

struct playback_callbacks {
    void (*notify)(sp_playback_notify_t type, void *userdata);
    void (*data)(void *samples, uint32_t num_frames, sp_audioformat *format, void *userdata);
    void (*seek)(uint32_t millis, void *userdata);
    void (*volume)(uint16_t volume, void *userdata);
};

struct debug_callbacks {
    void (*message)(char *msg, void *userdata);
};


sp_err_t SpInit(struct init_data *config);
sp_err_t SpZeroConfGetVars(struct vars_data *vars);

sp_err_t SpRegisterConnectionCallbacks(struct connection_callbacks *callbacks,
        void *userdata);
sp_err_t SpRegisterPlaybackCallbacks(struct playback_callbacks *callbacks,
        void *userdata);
sp_err_t SpRegisterDebugCallbacks(struct debug_callbacks *callbacks,
        void *userdata);
/*
 * volume: ranges from 0 to 65535
 */
sp_err_t SpPlaybackUpdateVolume(uint16_t volume);
sp_err_t SpConnectionLoginPassword(const char *login, const char *password);
sp_err_t SpConnectionLoginZeroConf(const char *username, const char *blob,
        const char *clientKey);
sp_err_t SpPumpEvents(void);

#endif
