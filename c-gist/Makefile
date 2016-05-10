CC := gcc
CXX := g++

CFLAGS += #--static
CLFAGS += -Wall -Wextra -Werror -ansi -fno-asm -O2 -g
LDFLAGS += -lm
GCCVERSIONGTEQ48 := $(shell expr `gcc -dumpversion ` \>= 4.8)

# only gcc >= 4.8, llvm >= 3.1 support AddressSanitizer
ifeq "$(GCCVERSIONGTEQ48)" "1"
    CFLAGS += -fsanitize=address -fno-omit-frame-pointer
endif

CSOURCE = $(wildcard *.c)
CPPSOURCE = $(wildcard *.cc)
#OBJS = $(patsubst %.c,%.o,$(CSOURCE))
TARGET = $(patsubst %.c,%.bin,$(CSOURCE)) $(patsubst %.cc,%.bin,$(CPPSOURCE))

.PHONY: all clean check

all:$(TARGET)

%.bin:%.c
	$(CC) $(CFLAGS) $(CLFAGS) $(LDFLAGS) $< -o $@

%.bin:%.cc
	$(CXX) $(CFLAGS) $(CLFAGS) $(LDFLAGS) $< -o $@

#all:$(objects)
#	gcc $^ -o $^.bin -ansi -fno-asm -O2 -Wall -lm --static

check:
	cppcheck --enable=all --std=c99 *.c

clean:
	rm -f *.bin *.o

# $(wildcard *.c source/*.c)
# wildcard把目录./和./source/下的所有后缀是c的文件全部展开

# $(patsubst %c, %o, $(SRCS))
# patsubst把$(SRCS)中的变量后缀是.c的全部替换成.o
