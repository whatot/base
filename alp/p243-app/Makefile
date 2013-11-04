CC=gcc
CFLAGS=-g -Wall -Wextra -Werror -pedantic -std=gnu99
LDFLAGS= -ldl

.PHONY: all test dist-clean clean GTAGS GTAGS-clean

sources := $(wildcard ./*.c)
objects := $(notdir $(sources:.c=.o))

run_main_obj := main.o
test_sources := $(wildcard ./*test*.c)
test_main_obj := $(notdir $(test_sources:.c=.o))
# test_main_obj contains all *test*.o
# but run_main_obj only contains the main.o

run_objects := $(filter-out $(test_main_obj),$(objects))
all: $(run_objects)
	$(CC) $^ $(CFLAGS) $(LDFLAGS) -o server.bin

test_objects := $(filter-out $(run_main_obj),$(objects))
test: $(test_objects)
	$(CC) $^ $(CFLAGS) $(LDFLAGS) -o test.bin
	./test.bin

%.bin: %.o
	$(CC) $^ $(CFLAGS) $(LDFLAGS) -o $@

dist-clean: clean GTAGS-clean

clean:
	-rm -f *.bin *.o

GTAGS:
	find . -type f -name '*.[ch]' > gtags.file
	gtags
	-rm -f gtags.file

GTAGS-clean:
	-rm -f gtags.file GPATH GRTAGS GTAGS
