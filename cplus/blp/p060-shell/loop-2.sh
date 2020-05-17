#!/bin/sh

for file in $(ls f*.sh); do
	echo
	cat $file
done
exit 0
