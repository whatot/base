
CUR_DIR=$(shell pwd)

CC := gcc
CPPFLAGS := -I$(CUR_DIR)/src
CFLAGS := $(CPPFLAGS) -g -std=c99 -Wall -Wextra -pedantic -O2 -Wno-unused -fPIC
#LDFLAGS := -L/usr/lib/
LIBS := -lm -rdynamic
# -rdynamic only function in link step, for backtrace
# search for headers when compile, -I, ENV, /usr/include/ ...
# search for libs when compile, -L, LIBRARY_PATH, /usr/lib ...
# search for libs when run time, '-Wl,-rpath', LD_LIBRARY_PATH, /etc/ld.so.conf, /usr/lib ...
VALGRIND_ARGV := --leak-check=full

default: all test

SRC := $(wildcard src/*.c)
OBJ = $(addprefix obj/,$(notdir $(SRC:.c=.o)))
FINAL_MAIN_OBJ := obj/ch_main.o
TEST_SRC := $(SRC) $(wildcard test/*.c)
TEST_OBJ = $(filter-out $(FINAL_MAIN_OBJ),$(addprefix obj/,$(notdir $(TEST_SRC:.c=.o))))
FINAL_TARGET := changing.bin
FINAL_TEST := test.bin

obj:
	mkdir -p obj/

all: $(OBJ)
	$(CC) $^ $(LIBS) -o $(FINAL_TARGET)

test: $(TEST_OBJ)
	$(CC) $^ $(LIBS) -o $(FINAL_TEST)

valgrind: all test
	valgrind $(VALGRIND_ARGV) ./$(FINAL_TARGET)

valgrind-test: all test
	valgrind $(VALGRIND_ARGV) ./$(FINAL_TEST)

obj/%.o: src/%.c | obj
	$(CC) $< -c $(CFLAGS) -o $@

obj/%.o: test/%.c | obj
	$(CC) $< -c $(CFLAGS) -o $@


obj/%.d: src/%.c
	set -e; rm -f $@; \
	$(CC) -MM $(CPPFLAGS) $< > $@.$$$$; \
	sed 's,\($*\)\.o[ :]*,\1.o $@ : ,g' < $@.$$$$ > $@; \
	rm -f $@.$$$$

obj/%.d: test/%.c
	set -e; rm -f $@; \
	$(CC) -MM $(CPPFLAGS) $< > $@.$$$$; \
	sed 's,\($*\)\.o[ :]*,\1.o $@ : ,g' < $@.$$$$ > $@; \
	rm -f $@.$$$$

# deal with the headers depends
include $(TEST_OBJ:.o=.d)
include $(FINAL_MAIN_OBJ:.o=.d)

GTAGS:
	find . -type f -name '*.[ch]' > gtags.files
	gtags
	rm -rf gtags.files

GTAGS-clean:
	-rm -rf gtags GPATH GRTAGS GTAGS gtags.files

clean:
	-rm -rf $(FINAL_MAIN_OBJ)
	-rm -rf $(TEST_OBJ)
	-rm -rf $(FINAL_TARGET) $(FINAL_TEST)

distclean: GTAGS-clean
	rm -rf obj/*

.PHONY: all GTAGS GTAGS-clean distclean clean valgrind valgrind-test
