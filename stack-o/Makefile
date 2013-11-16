CC := gcc
CXX := g++

CFLAGS += #--static
CLFAGS += -Wall -Wextra -ansi -fno-asm -O2 -g
LDFLAGS += -lm
GCCVERSIONGTEQ48 := $(shell expr `gcc -dumpversion ` \>= 4.8)

# only gcc >= 4.8, llvm >= 3.1 support AddressSanitizer
ifeq "$(GCCVERSIONGTEQ48)" "1"
    CFLAGS += -fsanitize=address -fno-omit-frame-pointer
endif

SOURCE = $(wildcard *.c)
#OBJS = $(patsubst %.c,%.o,$(SOURCE))
TARGET = $(patsubst %.c,%.bin,$(SOURCE))

.PHONY: all clean

all:$(TARGET)

%.bin:%.c
	$(CC) $(CFLAGS) $(CLFAGS) $(LDFLAGS) $< -o $@

#all:$(objects)
#	gcc $^ -o $^.bin -ansi -fno-asm -O2 -Wall -lm --static

clean:
	rm -f *.bin *.o

# $(wildcard *.c source/*.c)
# wildcard把目录./和./source/下的所有后缀是c的文件全部展开

# $(patsubst %c, %o, $(SRCS))
# patsubst把$(SRCS)中的变量后缀是.c的全部替换成.o
