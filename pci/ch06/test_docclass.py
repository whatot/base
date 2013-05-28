#!/usr/bin/python2
# -*- coding:utf-8 -*-

import unittest
import docclass


class test_docclass(unittest.TestCase):
    def setup(self):
        pass

    def teardown(self):
        pass

    def test_getwords(self):
        words_source = 'disk*jdhs&342nhek[]989c12cnjshdsjkv774*j'
        words_result_dict = dict([('disk', 1), ('jdhs', 1),
                                 ('nhek', 1), ('cnjshdsjkv', 1)])
        self.assertDictEqual(words_result_dict,
                             docclass.getwords(words_source))

    def test_incf(self):
        clas = docclass.classifier(docclass.getwords)
        origin_str = 'the quick brown for jumps over the lazy dog'
        clas.train('the quick brown for jumps over the lazy dog', 'good')
        str_tobe = docclass.getwords(origin_str)
        self.assertDictEqual(clas.fc, str_tobe)

if __name__ == '__main__':
    unittest.main()
