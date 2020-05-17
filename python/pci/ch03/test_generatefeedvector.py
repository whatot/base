# -*- coding:utf-8 -*-
#!/usr/bin/python

import unittest
import urllib
import generatefeedvector


class Testgeneratefeedvector(unittest.TestCase):

    def setUp(self):
        print("test start")

    def tearDown(self):
        print("test stop")

    def testgetwords(self):
        url_file = urllib.urlopen("http://www.google.com.hk").readlines()
        self.assertEqual(generatefeedvector.getwords(url_file), [])
        url_file.close()

    def testgetwordcounts(self):
        url = ("http://www.google.com.hk")
        self.assertEqual(generatefeedvector.getwordcounts(url), {})


if __name__ == '__main__':
    unittest.main()
