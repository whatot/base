#include <assert.h>
#include <getopt.h>
#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "server.h"

/* Description of long options for getopt_long */
static const struct option long_options[] = {
	{ "address",        1, NULL, 'a' },
	{ "help",            0, NULL, 'h' },
	{ "module--dir",    1, NULL, 'm' },
	{ "port",            1, NULL, 'p' },
	{ "verbose",        0, NULL, 'v' },
};

/* Description of short options for getopt_long */
static const char *const short_options = "a:hm:p:v";

/* Usage summary text */
static const char *const usage_template =
"Usage: %s [ options ]\n"
"    -a, --address ADDR        Bind to local address (by default, bind\n"
"                                  to all local addresses).\n"
"    -h, --help                Print this information.\n"
"    -m, --module-dir DIR      Load Modules from specified directory\n"
"                                  (by default, use executable directory).\n"
"    -p, --port PORT           Bind to specified port.\n"
"    -v, --verbose             Print verbose messages.\n";

/* Print usage information and exit. If IS_ERROR is nonzero, write to
 * stderr and use an error exit code. Otherwise, write to stdout and
 * use a non-error termination code. Does not return. */
static void print_usage(int is_error)
{
	fprintf(is_error ? stderr : stdout, usage_template, program_name);
	exit(is_error ? 1 : 0);
}

int main(int argc, char * const argv[]) {
	struct in_addr local_address;
	uint16_t port;
	int next_option;

	/* Store the program name, which we'll use in error messages. */
	program_name = argv[0];

	/* Set defaults for options. Bind the server to all local addresses,
	 * and assign an unused port automatically. */
	local_address.s_addr = INADDR_ANY;
	port = 0;
	/* Don't print verbose messages. */
	verbose = 0;
	/* Load modules from the directory containing this executable. */
	module_dir = get_self_executable_directory();
	assert(module_dir != NULL);

	/* Parse options. */
	do {
		next_option =
			getopt_long(argc, argv, short_options, long_options, NULL);
		switch (next_option) {
		case 'a':
			/* User specified -a or --address. */
			{
				struct hostent *local_host_name;

				/* Look up the hostname the user specified. */
				local_host_name = gethostbyname(optarg);
				if (local_host_name == NULL ||
					local_host_name->h_length == 0) {
					error(optarg, "invalid host name");
				} else {
					/* Hostname is OK, so use it.  */
					local_address.s_addr =
						*((int *)(local_host_name->h_addr_list[0]));
				}
			}
			break;

		case 'h':
			/* User specified -h and --help */
			print_usage(0);

		case 'm':
			/* Use specified -m or --module-dir.  */
			{
				struct stat dir_info;

				/* Check that it exists. */
				if (access(optarg, F_OK) != 0) {
					error(optarg, "module directory does not exists");
				}
				/* Check that it is accessible. */
				if (access(optarg, R_OK | X_OK) != 0) {
					error(optarg, "module directory is not accessible");
				}
				/* Make sure that it is a directory. */
				if (stat(optarg, &dir_info) != 0 ||
					!S_ISDIR(dir_info.st_mode)) {
					error(optarg, "not a directory");
				}
				/* It looks OK, so use it. */
				module_dir = xstrdup(optarg);
			}
			break;

		case 'p':
			/* User specified -p or --port. */
			{
				long value;
				char *end;

				value = strtol(optarg, &end, 10);
				if (*end != '\0') {
					/* The user specified nondigits in the port number. */
					print_usage(1);
				}
				/* The port number needs to be converted to network
				 * (big endian) byte order. */
				port = (uint16_t)htons(value);
			}
			break;

		case 'v':
			/* User specified -v or --verbose. */
			verbose = 1;
			break;

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

	/* Print the module directory, if we're running verbose. */
	if (verbose) {
		printf("modules will be loaded from %s\n", module_dir);
	}

	/* Run the server. */
	server_run(local_address, port);

	return 0;
}
