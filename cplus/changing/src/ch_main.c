#include <signal.h>
#include "changing.h"
#include "ch_start.h"
#include "ch_exit.h"

int main(int argc, char *argv[])
{
	signal(SIGINT, ch_exit);
	signal(SIGSEGV, ch_exit);

	ch_start(argc, argv);

	ch_exit();

	return 0;
}
