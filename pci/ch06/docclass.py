#!/usr/bin/python2
# -*- coding:utf-8 -*-
'''
(?x)            # set flag to allow verbose regexps
([A-Z]\.)+         # abbreviations, e.g. U.S.A.
| \w+(-\w+)*         # words with optional internal hyphens
| \$?\d+(\.\d+)?%?   # currency and percentages, e.g. $12.40, 82%
| \.\.\.              # ellipsis
| [][.,;"'?():-_`]     # these are separate tokens
'''

import re
import math


def getwords(doc):
    doc_oneline = doc.replace('[\s\t\n]+', ' ')
    pattern = '''[a-zA-Z]+'''
    splitter = re.compile(pattern)
    # 根据非字母字符进行单词拆分
    words = [s.lower() for s in splitter.findall(doc_oneline)
             if len(s) > 2 and len(s) < 20]

    # 只返回一组不重复的单词
    return dict([(w, 1) for w in words])
