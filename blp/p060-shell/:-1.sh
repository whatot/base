#!/bin/sh

rm -rf fred

if [ -f fred ]; then
	:
else
	echo file fred did not exist
fi

exit 0
