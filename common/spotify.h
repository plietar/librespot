#ifndef SPOTIFY_H
#define SPOTIFY_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

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
    kSpConnectionNotifyLoggedOut = 1,
} sp_connection_notify_t;

typedef enum {
    kSpPlaybackNotifyPlay = 0,
    kSpPlaybackNotifyPause = 1,
    kSpPlaybackNotifyTrackChanged = 2,
    kSpPlaybackNotifyShuffleEnabled = 5,
    kSpPlaybackNotifyShuffleDisabled = 6,
    kSpPlaybackNotifyRepeatEnabled = 7,
    kSpPlaybackNotifyRepeatDisabled = 8,
    kSpPlaybackNotifyBecameActive = 9,
    kSpPlaybackNotifyBecameInactive = 10,
    kSpPlaybackNotifyPlayTokenLost = 11,
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

/* Not sure about the order */
typedef enum {
    kSpBitrate160 = 0,
    kSpBitrate320 = 1,
    kSpBitrate90 = 2
} sp_bitrate_t;

typedef enum {
    kSpImageSize160 = 0,
    kSpImageSize320 = 1,
    kSpImageSize640 = 2
} sp_image_size_t;

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
    char deviceId[0x40];
    char activeUser[0x40];
    char remoteName[0x40];
    char accountReq[0x10];
    char deviceType[0x10];
};

struct SpMetadata {
    char data0[0x100];
    char context_uri[0x80];
    char track_name[0x100];
    char track_uri[0x80];
    char artist_name[0x100];
    char artist_uri[0x80];
    char album_name[0x100];
    char album_uri[0x80];
    char cover_uri[0x80];
    uint32_t duration;
};

struct connection_callbacks {
    void (*notify)(sp_connection_notify_t type, void *userdata);
    void (*message)(const char *msg, void *userdata);
};

struct playback_callbacks {
    void (*notify)(sp_playback_notify_t type, void *userdata);
    void (*data)(const void *samples, uint32_t num_frames,
            sp_audioformat *format, void *userdata);
    void (*seek)(uint32_t millis, void *userdata);
    void (*volume)(uint16_t volume, void *userdata);
};

struct debug_callbacks {
    void (*message)(const char *msg, void *userdata);
};


sp_err_t SpInit(const struct init_data *config);
void SpFree(void);

sp_err_t SpPumpEvents(void);

int SpGetMetadataValidRange(int *start, int *end);
int SpGetMetadata(struct SpMetadata *, int offset);
int SpGetMetadataImageURL(const char *uri, sp_image_size_t image_size,
        char *url, size_t size);

int SpGetPreset(void *, void *);
void SpSetPreset(void *);


sp_err_t SpSetDisplayName(const char *name);
const char *SpGetLibraryVersion(void);


sp_err_t SpZeroConfGetVars(struct vars_data *vars);


sp_err_t SpPlaybackPlay(void);
sp_err_t SpPlaybackPause(void);
sp_err_t SpPlaybackSkipToNext(void);
sp_err_t SpPlaybackSkipToPrev(void);
sp_err_t SpPlaybackSeek(uint32_t millis);
sp_err_t SpPlaybackUpdateVolume(uint16_t volume);
uint16_t SpPlaybackGetVolume(void);
int SpPlaybackIsPlaying(void);
int SpPlaybackIsShuffled(void);
int SpPlaybackIsRepeated(void);
int SpPlaybackIsActiveDevice(void);
sp_err_t SpPlaybackEnableShuffle(bool enable);
sp_err_t SpPlaybackEnableRepeat(bool enable);
sp_err_t SpPlaybackSetBitrate(sp_bitrate_t bitrate);

sp_err_t SpConnectionLoginPassword(const char *login, const char *password);
sp_err_t SpConnectionLoginZeroConf(const char *username, const char *blob,
        const char *clientKey);
sp_err_t SpConnectionLoginOauthToken(const char *token);

int SpConnectionIsLoggedIn();
sp_err_t SpConnectionLogout();


sp_err_t SpRegisterConnectionCallbacks(
        const struct connection_callbacks *callbacks, void *userdata);
sp_err_t SpRegisterPlaybackCallbacks(
        const struct playback_callbacks *callbacks, void *userdata);
sp_err_t SpRegisterDebugCallbacks(
        const struct debug_callbacks *callbacks, void *userdata);




#endif
