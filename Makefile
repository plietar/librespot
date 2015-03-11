CC=arm-linux-gnueabi-gcc-4.9

all: main

main: spotify.h audio.h
main: main.c audio.c
	$(CC) -Wall -std=c11 -L. -lspotify_embedded_shared -lm -D_GNU_SOURCE main.c audio.c -o $@

clean:
	rm -f lib.so lib.o main main.o
