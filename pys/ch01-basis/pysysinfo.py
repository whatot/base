#!/usr/bin/python
# A system information gathering script
# Usage:
# :import pysysinfo
# :pysysinfo.uname_func
# :pysysinfo.main

import subprocess

# command 1
def uname_func():
    uname = "uname"
    uname_arg = "-a"
    print "Gathering system information with %s command:\n" % uname
    subprocess.call([uname, uname_arg])

# command 2
def disk_func():
    diskspace = "df"
    diskspace_arg = "-h"
    print "Gathering diskspace information with %s command:\n" % diskspace
    subprocess.call([diskspace, diskspace_arg])

# Main function that call other functions
def main():
    uname_func()
    disk_func()

if __name__ == "__main__":
    main()
