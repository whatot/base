#!/usr/bin/python2.7

import sys


for i, line in enumerate(sys.stdin):
    print("%s: %s" % (i, line))
