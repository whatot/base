#include <stdio.h>
#include <sys/shm.h>
#include <sys/ipc.h>
#include <sys/stat.h>

int main(int argc, const char *argv[])
{
	int segment_id;
	char* shared_memory;
	struct shmid_ds shmbuffer;
	int segment_size;
	const int shared_segment_size = 0x6400;

	/* Allocate a shared_memory segment.  */
	segment_id = shmget (IPC_PRIVATE, shared_segment_size,
			     IPC_CREAT | IPC_EXCL | S_IRUSR | S_IWUSR);
	/* Attach the shared_memory segment. */
	shared_memory = (char *) shmat (segment_id, 0, 0);
	printf ("shared_memory attached at address %p\n", shared_memory);
	/* Determine the segment's size.  */
	shmctl (segment_id, IPC_STAT, &shmbuffer);
	segment_size = shmbuffer.shm_segsz;
	printf("segment_size: %d\n", segment_size);
	/* Write a string to the shared_memory segment.  */
	sprintf (shared_memory, "Hello, world.");
	/* Detach the shared_memory segment.  */
	shmdt (shared_memory);

	/* Reattach the shared_memory segment, at a different address.  */
	shared_memory = (char *) shmat (segment_id, (void *) 0x5000000, 0);
	printf("shared_memory reattached at address %p\n", shared_memory);
	/* Print out the string from shared_memory.  */
	printf("%s\n", shared_memory);
	/* Detach the shared_memory segment.  */
	shmdt (shared_memory);

	/*Deallocate the shared_memory segment. */
	shmctl (segment_id, IPC_RMID, 0);

	return 0;
}
