CFLAGS=-Wall -Wextra -g -DNDEBUG -O2 # -Werror

objects = ex1 ex3 ex5 ex6 ex7 ex8 ex9 ex10 ex11 ex12 ex13 ex14 ex15 ex16 ex17 ex18 ex20 ex22_main ex23 ex24 ex25
targetbin = $(addsuffix .bin,$(objects))

%.bin:%.c
	$(CXX) $(CFLAGS) $(CLFAGS) $(LDFLAGS) $^ -o $@

.PHONY:all
all:$(targetbin)

# ex19:object.o

ex22_main.bin:ex22.c ex22_main.c

.PHONY:clean

clean:
	-rm -f $(targetbin)
