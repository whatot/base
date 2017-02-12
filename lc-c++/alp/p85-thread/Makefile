CC=gcc
CFLAGS=-g -Wall
LDFLAGS=-lpthread

.PHONY: all clean

exclude_files:=spin-condvar.bin job-queue1.bin job-queue2.bin \
	cleanup.bin critical-section.bin job-queue3.bin

sources:=$(wildcard ./*.c)
all_objects:=$(notdir $(sources:.c=.bin))
objects:= $(filter-out $(exclude_files),$(all_objects))

all:$(objects)

%.bin: %.c
	$(CC) $^ $(CFLAGS) $(LDFLAGS) -o $@

clean:
	-rm -f $(objects)
