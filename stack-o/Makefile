objects=$(wildcard *.c)

.PHONY: all clean

all:$(objects)
	gcc $^ -o $^.bin -ansi -fno-asm -O2 -Wall -lm --static

clean:
	rm -f *.bin

# $(wildcard *.c source/*.c)
# wildcard把目录./和./source/下的所有后缀是c的文件全部展开

# $(patsubst %c, %o, $(SRCS))
# patsubst把$(SRCS)中的变量后缀是.c的全部替换成.o
