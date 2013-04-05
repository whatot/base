CC=gcc
CFLAGS=-g -Wall
LDFLAGS=

objects=%.c

.PHONY:all
all:$(objects)


.PHONY:clean
clean:
	-rm $(objects) *.o
