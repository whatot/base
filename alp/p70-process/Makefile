CC=gcc
CFLAGS=-g -Wall
LDFLAGS=

sources=$(wildcard ./*.c)
objects=$(notdir $(sources:.c=.bin))

.PHONY:all
all:$(objects)

%.bin: %.c
	$(CC) $^ $(CFLAGS) $(LDFLAGS) -o $@

.PHONY:clean
clean:
	-rm -f $(objects)
