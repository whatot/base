CC=gcc
CFLAGS=-g -Wall -Wextra -Werror -pedantic -std=c99
LDFLAGS=

exclude_files:=
sources:=$(wildcard ./*.c)
all_objects:=$(notdir $(sources:.c=.bin))
objects:= $(filter-out $(exclude_files),$(all_objects))

.PHONY:all
all:$(objects)

%.bin: %.c
	$(CC) $^ $(CFLAGS) $(LDFLAGS) -o $@

.PHONY:clean
clean:
	-rm -f $(objects)
