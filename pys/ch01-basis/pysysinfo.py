#!/usr/bin/python
# A system information gathering script
import subprocess

# command 1
uname = "uname"
uname_arg = "-a"
print "Gathering system information with %s command:\n" % uname
subprocess.call([uname, uname_arg])

# command 2
diskspace = "df"
diskspace_arg = "-h"
print "Gathering diskspace information with %s command:\n" % diskspace
subprocess.call([diskspace, diskspace_arg])
