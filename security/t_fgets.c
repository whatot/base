#include <stdio.h>
#define BUFFSIZE 20

int main(void) {
	char buff[BUFFSIZE];
	if (fgets(buff, BUFFSIZE, stdin) == NULL) {
		printf("read error.\n");
		return 1;
	}
	printf("fgets: %s.\n",buff);

	return 0;
}
