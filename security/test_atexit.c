#include <stdio.h>

char *glob;
void test(void) {
	printf("%s", glob);
}

int main(void) {
	atexit(test);
	glob = "Exiting.\n";
	return 0;
}

