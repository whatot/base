#include <sys/wait.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

// from "man pipe"
// $ gcc -o man_pipe man_pipe.c
// $ ./man_pipe hjshjhcd

int main(int argc, char const* argv[])
{
	int pipefd[2];
	pid_t cpid;
	char buf;

	if (argc != 2) {
		fprintf(stderr, "Usage:%s <string>\n",argv[0]);
		exit(EXIT_FAILURE);
	}

	if (pipe(pipefd) == -1) {
		perror("pipe");
		exit(EXIT_FAILURE);
	}

	cpid = fork();
	if (cpid == -1) {
		perror("fork");
		exit(EXIT_FAILURE);
	}

	if (cpid == 0) {	/* Child reads from pipe. */
		close(pipefd[1]);	/* Close unused write end. */
		fprintf(stderr, "child pid: %d\n", (int)getpid());

		while (read(pipefd[0], &buf, 1) > 0) {
			fprintf(stderr, "child read from pipe\n");
			write(STDOUT_FILENO, &buf, 1);
		}

		write(STDOUT_FILENO, "\n", 1);
		close(pipefd[0]);
		_exit(EXIT_SUCCESS);

	} else {	/* Parent writes argv[1] to pipe. */
		close(pipefd[0]);	/* Close unused read end. */
		fprintf(stderr, "parent pid: %d\n", (int)getpid());

		write(pipefd[1], argv[1], strlen(argv[1]));
		close(pipefd[1]);	/* Reader will see EOF. */
		wait(NULL);		/* Wait for child. */
		exit(EXIT_SUCCESS);
	}
        return 0;
}
