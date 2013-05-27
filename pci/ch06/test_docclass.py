#!/usr/bin/python2
# -*- coding:utf-8 -*-

import unittest
import docclass


class test_docclass(unittest.TestCase):
    def test_getwords(self):
        words_source = 'disk*jdhs&342nhek[]989c12cnjshdsjkv774*j'
        words_result_dict = dict([('disk', 1), ('jdhs', 1),
                                 ('nhek', 1), ('cnjshdsjkv', 1)])
        self.assertDictEqual(words_result_dict,
                             docclass.getwords(words_source))


if __name__ == '__main__':
    unittest.main()
