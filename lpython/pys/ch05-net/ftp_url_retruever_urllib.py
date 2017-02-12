#!/usr/bin/python2
# -*- coding:utf-8 -*-
'''
ftp_url_retruever_urllib.py

Usage:
    $ python2 ftp_url_retruever_urllib.py URL FILENAME

Example:
    $ python2 ftp_url_retruever_urllib.py http://www.sina.com.cn ./sina.html

 - URL: like http://mirrors6.ustc.edu.cn/centos/6.4/isos/i386/sha256sum.txt
    where is the file which you want place

 - FILENAME: the name which the file you want download would be saved as
'''

import urllib
import sys


if '-h' in sys.argv or '--help' in sys.argv:
    print(__doc__)
    sys.exit(1)

if not len(sys.argv) == 3:
    print('URL and filename are mandatory')
    print(__doc__)
    sys.exit(1)


def cbk(a, b, c):
    '''''回调函数
    @a: 已经下载的数据块
    @b: 数据块的大小
    @c: 远程文件的大小
    '''
    per = 100.0 * a * b / c
    if per > 100:
        per = 100
    print('%.2f%%' % per)

# url = 'http://www.sina.com.cn'
# local = 'd://sina.html'
# urllib.urlretrieve(url, local, cbk)

url = sys.argv[1]
filename = sys.argv[2]
urllib.urlretrieve(url, filename, cbk)
