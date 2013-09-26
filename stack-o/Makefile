CC=gcc
CXX=g++

CFLAGS += --static
CFLAGS += -Wall -ansi -fno-asm -O2 -g
LDFLAGS += -lm

SOURCE = $(wildcard *.c)
TARGET = $(patsubst %.c,%,$(SOURCE))

.PHONY: all clean

all:$(TARGET)

%.bin:%.c
	$(CXX) $(CFLAGS) $(LDFLAGS) $< -o $@

clean:
	rm -f $(TARGET) *.o

# $(wildcard *.c source/*.c)
# wildcard把目录./和./source/下的所有后缀是c的文件全部展开

# $(patsubst %c, %o, $(SRCS))
# patsubst把$(SRCS)中的变量后缀是.c的全部替换成.o
