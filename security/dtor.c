#include <stdio.h>
#include <stdlib.h>

static void create(void)
	__attribute__ ((constructor));
static void destroy(void)
	__attribute__ ((destructor));

int main(int argc, char *argv[]) {
	printf("create: %p.\n",create);
	printf("destroy: %p.\n",destroy);
	exit(EXIT_SUCCESS);
}
void create(void) {
	printf("create called.\n");
}
void destroy(void) {
	printf("destroy called.\n");
}

