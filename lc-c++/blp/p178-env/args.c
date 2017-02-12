#include <stdio.h>

int main(int argc, char const* argv[])
{
	int arg;

	for (arg = 0; arg < argc; arg++) {
		if (argv[arg][0] == '-') {
			printf("option: %s\n", argv[arg]+1);
		} else {
			printf("argument %d: %s\n", arg, argv[arg]);
		}
	}
	return 0;
}
