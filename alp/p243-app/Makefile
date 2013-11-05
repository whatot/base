CC=gcc
CFLAGS=-g -Wall -Wextra -Werror -pedantic -std=gnu99
LDFLAGS= -ldl

.PHONY: all test dist-clean clean GTAGS GTAGS-clean

COMMON_OBJECTS := server.o module.o common.o
MAIN_OBJECTS := main.o $(COMMON_OBJECTS)
MODULES_OBJECTS := diskfree.so issue.so time.so
TEST_SOURCES := $(wildcard ./*test*.c)
TEST_OBJECTS := $(notdir $(TEST_SOURCES:.c=.o)) $(COMMON_OBJECTS)

all: server $(MODULES_OBJECTS)

server: $(MAIN_OBJECTS)
	$(CC) $^ $(CFLAGS) $(LDFLAGS) -o server.bin

$(MODULES_OBJECTS): \
%.so: %.c server.h
	$(CC) $(CFLAGS) -fPIC -shared -o $@ $<

test: $(TEST_OBJECTS)
	$(CC) $^ $(CFLAGS) $(LDFLAGS) -o test.bin
	./test.bin

%.bin: %.o
	$(CC) $^ $(CFLAGS) $(LDFLAGS) -o $@

clean:
	-rm -f *.bin *.o *.so

GTAGS:
	find . -type f -name '*.[ch]' > gtags.file
	gtags
	-rm -f gtags.file

GTAGS-clean:
	-rm -f gtags.file GPATH GRTAGS GTAGS

dist-clean: clean GTAGS-clean
