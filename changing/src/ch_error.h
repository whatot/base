#ifndef ch_error_h
#define ch_error_h

#include <stdio.h>				/* fprintf */
#include <stdlib.h>				/* abort() */
#include <unistd.h>				/* STDOUT_FILENO */
#include <execinfo.h>			/* backtrace for system_error_exit */
#include "ch_config.h"

#define MAX_BACKTRACE_BUF_SISZ (20480)
#define MAX_BACKTRACE_LEVEL 20
void *MAX_BACKTRACE_BUF[MAX_BACKTRACE_BUF_SISZ];

/* if this happen, it should output why error and then exit */
#if defined DONOT_USE_SYSTEM_ERRROR_EXIT
#define system_error_exit()
#else
#define system_error_exit() \
	do { int size = backtrace((void **)MAX_BACKTRACE_BUF,MAX_BACKTRACE_BUF_SISZ); \
		if (size == MAX_BACKTRACE_BUF_SISZ) { \
			fprintf(stderr, "level: %d may be to small\n", size); \
		}\
	backtrace_symbols_fd(MAX_BACKTRACE_BUF, size, STDOUT_FILENO); \
	abort(); } while(0)
#endif

#endif
