#include "ch_start.h"

#include <getopt.h>
#include <stdio.h>
#include <unistd.h>
#include "changing.h"
#include "ch_config.h"
#include "ch_status.h"
#include "ch_log.h"

static char *program_name;

/* Description of long options for getopt_long */
static const struct option long_options[] = {
	{"logtype", 1, NULL, 'l'},
	{"loglevel", 1, NULL, 'd'},
	{"help", 0, NULL, 'h'},
};

/* Description of short options for getopt_long */
static const char *const short_options = "l:d:h";

static const char *const usage_template =
	"Usage: %s [ options ]\n"
	"    -l, --logtype 1-3         (1,2,3) log_STDOUT,log_SYSLOG,log_FILE \n"
	"    -d, --loglevel 1-7        (1..7) LOG_ERR ... LOG_DEBUG\n"
	"    -h, --help                Print this information.\n";

static void print_usage(int is_error)
{
	fprintf(is_error ? stderr : stdout, usage_template, program_name);
	exit(is_error ? 1 : 0);
}

int ch_start(int argc, char *argv[])
{
	int next_option;
	int init_log_type = DEFAULT_INIT_LOG_TYPE;
	int init_log_level = DEFAULT_INIT_LOG_LEVEL;
	char *init_log_file = NULL;

	/* Store the program name, which we'll use in error messages. */
	program_name = argv[0];

	/* parse options */
	do {
		next_option =
			getopt_long(argc, argv, short_options, long_options, NULL);
		switch (next_option) {
		case 'l':
			{
				int log_type = atoi(optarg);
				if (log_type >= 1 && log_type <= 3) {
					init_log_type = log_type;
				}
				if (log_type == 3) {
					init_log_file = DEFAULT_INIT_LOG_FILE;
				}
				break;
			}
		case 'd':
			{
				int log_level = atoi(optarg);
				if (log_level > 7) {
					init_log_level = 7;
				} else if (log_level < 0) {
					init_log_level = 0;
				} else {
					init_log_level = log_level;
				}
				break;
			}
		case 'h':
			/* User specified -h and --help */
			print_usage(0);

		case '?':
			/* User specified an unrecognized option. */
			print_usage(1);

		case -1:
			/* Done with options */
			break;

		default:
			abort();
		}
	} while (next_option != -1);

	ch_log_init(init_log_level, init_log_type, init_log_file, NULL);

	int ret = ch_status_init();

	return !ret ? EXIT_SUCCESS : EXIT_FAILURE;
}
