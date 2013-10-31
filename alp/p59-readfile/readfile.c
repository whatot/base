/*
 *        Created:  2013年02月05日 14时58分06秒
 */

#include <fcntl.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>


char* read_from_file (const char* filename, size_t length)
{
	char* buffer;
	int fd;
	ssize_t bytes_read;

	/* Allocate the buffer. */
	buffer = (char *) malloc (length);
	if (buffer == NULL)
		return NULL;
	/* open the file. */
	fd = open (filename, O_RDONLY);
	if (fd == -1) {
		/* open failed, Deallocate buffer before returning. */
		free (buffer);
		return NULL;
	}
	/* Read the data.  */
	bytes_read = read (fd, buffer, length);
	if ((size_t)bytes_read != length) {
		/* read failed. Deallocate buffer and close fd before returning. */
		free (buffer);
		close (fd);
		return NULL;
	}
	/* Everything's fine. close the file and return the buffer.  */
	close (fd);
	return buffer;
}
