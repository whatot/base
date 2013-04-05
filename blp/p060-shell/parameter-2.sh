#!/bin/sh

unset foo
echo ${foo:-bar}

foo=fun
echo ${foo:-bar}

foo=/usr/bin/X11/startx
echo ${foo#*/}
echo ${foo##*/}

bar=/usr/local/etc/local/neworks
echo ${bar%local*}
echo ${bar%%local*}

exit 0
