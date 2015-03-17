CC=arm-linux-gnueabi-gcc-4.9
CFLAGS=-Wall -g -std=c11 -D_GNU_SOURCE
LDFLAGS=-I../common -L../common -lspotify_embedded_shared -lm -ldl

ifdef TARGET
CLEAN+=$(TARGET)
all: $(TARGET)
$(TARGET): $(SOURCES) $(HEADERS)
	$(CC) $(CFLAGS) $(LDFLAGS) $(SOURCES) -o $@
endif

all:
	make -C common

clean:
	rm -f $(CLEAN)

.PHONY: all clean
