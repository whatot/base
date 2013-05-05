#!/usr/bin/python2.7
# example:
# $ who | python sys_stdin_readline.py

import sys

counter = 1

while True:
    line = sys.stdin.readline()
    if not line:
        break
    print("%s: %s" % (counter, line))
    counter += 1
