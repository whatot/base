#include <stdio.h>
#include <string.h>

int main(int argc, char *argv[]) {
	char a[16];
	char b[16];
	char c[32];

	strcpy(a, "0123456789abcdef");
	strcpy(b, "0123456789abcdef");
	strcpy(c, a);
	strcpy(c, b);
	printf("a = %s\n", a);
	printf("b = %s\n", b);
	printf("c = %s\n", c);
	return 0;
}

