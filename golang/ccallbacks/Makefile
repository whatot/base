build: clibrary
	go build

clibrary:
	gcc -c clibrary.c
	ar cr libclibrary.a clibrary.o

clean:
	rm clibrary.o libclibrary.a ccallbacks

.PHONY: clibrary build clean
