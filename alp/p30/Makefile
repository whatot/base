CC=gcc
CFLAGS=-g -Wall -Wextra -Werror -pedantic -std=c99
CXXFLAGS=-g -Wall -Wextra -Werror -pedantic -std=c++11
LDFLAGS=

.PHONY: clean

reciprocal.bin:main.o reciprocal.o
	g++ $(CXXFLAGS) -o reciprocal.bin main.o reciprocal.o

main.o:main.c reciprocal.hpp
	gcc $(CFLAGS) -c main.c

reciprocal.o:reciprocal.cpp reciprocal.hpp
	g++ $(CXXFLAGS) -c reciprocal.cpp

clean:
	-rm -f *.o reciprocal.bin
