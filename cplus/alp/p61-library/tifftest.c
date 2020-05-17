/*
 *        Created:  2013年02月05日 17时37分03秒
 */

#include <stdio.h>
#include <tiff.h>

int main(int argc, const char **argv)
{
	TIFF* tiff;
	tiff = TIFFOpen (argv[1], "r");
	TIFFClose (tiff);

	return 0;
}


//$ gcc -o tifftest tifftest.c -ltiff
///usr/lib/libtiff.so 依赖于libjpeg与libz
//$ gcc -static -o tifftest tifftest.c -ltiff -ljpeg -lz
