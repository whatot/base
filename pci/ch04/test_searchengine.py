#!/usr/bin/python2
# -*- coding:utf-8

import urllib2
from bs4 import BeautifulSoup
import unittest
import searchengine


class Testsearchengine(unittest.TestCase):

    def setUp(self):
        print("test start")

    def tearDown(self):
        print("test end")

    def testseparatewords(self):
        craw = searchengine.crawler('searchindex.db')
        page = "http://forum.ubuntu.org.cn/index.php"
        c = urllib2.urlopen(page)
        soup = BeautifulSoup(c.read())
        text = craw.gettextonly(soup)
        print(text)
        words = craw.separatewords(text)
        wordsneedtobe = []
        self.assertEqual(words[100:120], wordsneedtobe[:])
        pass

if __name__ == '__main__':
    unittest.main()
