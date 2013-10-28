CC=gcc
CXX=gcc

CFLAGS += #--static
CLFAGS += -Wall -Wextra -ansi -fno-asm -O2 -g -fsanitize=address -fno-omit-frame-pointer
LDFLAGS += -lm

SOURCE = $(wildcard *.c)
#OBJS = $(patsubst %.c,%.o,$(SOURCE))
TARGET = $(patsubst %.c,%.bin,$(SOURCE))

.PHONY: all clean

all:$(TARGET)

%.bin:%.c
	$(CXX) $(CFLAGS) $(CLFAGS) $(LDFLAGS) $< -o $@

#all:$(objects)
#	gcc $^ -o $^.bin -ansi -fno-asm -O2 -Wall -lm --static

clean:
	rm -f *.bin *.o

# $(wildcard *.c source/*.c)
# wildcard把目录./和./source/下的所有后缀是c的文件全部展开

# $(patsubst %c, %o, $(SRCS))
# patsubst把$(SRCS)中的变量后缀是.c的全部替换成.o
