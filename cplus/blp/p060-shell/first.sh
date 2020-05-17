#!/bin/sh

# first
# This file looks through all the files in current directory for
# the string POSIX, and then prints the names of those files to
# the standard output.

for file in *
do
	if grep -q POSIX $file
	then
		echo $file
	fi
done

exit 0
