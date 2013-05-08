#!/usr/bin/python2

import subprocess
# import re
import sys


def arping(ipaddress="10.0.1.1"):
    """Arping function takes IP Address or Network,return nested mac/ip list"""

    # Assuming use of arping on linux
    p = subprocess.Popen("/usr/sbin/arping -c 2 %s" % ipaddress, shell=True,
                         stdout=subprocess.PIPE)
    out = p.stdout.read()
    result = out.split()
    # pattern = re.compile(":")
    for item in result:
        if ':' in item:
            print(item)


if __name__ == '__main__':
    if len(sys.argv) > 1:
        for ip in sys.argv[1:]:
            print("arping", ip)
            arping(ip)
    else:
        arping()
