#include <stdio.h>

int main(void) {
	char *bp;
	size_t size;
	FILE *stream;

	//fmemopen()函数打开一个流,允许你读取或写入一个指定的缓存区.
	//open_memstream()函数则打开一个流用于向缓存区写入数据.
	stream = open_memstream(&bp, &size);
	fprintf(stream, "hello");
	fflush(stream); //刷新流,更新了buf和size,size表示缓冲区总计大小
	printf("buf = '%s', size = %d\n", bp, size);
	fprintf(stream, ", world");
	fclose(stream); //关闭流,也更新了buf和size
	printf("buf = '%s', size = %d\n", bp, size);

	return 0;
}

