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
# import math


def getwords(doc):
    doc_oneline = doc.replace('[\s\t\n]+', ' ')
    pattern = '''[a-zA-Z]+'''
    splitter = re.compile(pattern)
    # 根据非字母字符进行单词拆分
    words = [s.lower() for s in splitter.findall(doc_oneline)
             if len(s) > 2 and len(s) < 20]

    # 只返回一组不重复的单词
    return dict([(w, 1) for w in words])


class classifier:
    def __init__(self, getfeatures, filename=None):
        # 统计特征/分类组合的数量
        self.fc = {}
        # 统计每个分类中的文档数量
        self.cc = {}
        self.getfeatures = getfeatures

    # 增加对特征/分类组合的计数值
    def incf(self, f, cat):
        self.fc.setdefault(f, {})
        self.fc[f].setdefault(cat, 0)
        self.fc[f][cat] += 1

    # 增加对某一分类的计数值
    def incc(self, cat):
        self.cc.setdefault(cat, 0)
        self.cc[cat] += 1

    # 某一特征出现于某一分类中的次数
    def fcount(self, f, cat):
        if f in self.fc and cat in self.fc[f]:
            return float(self.fc[f][cat])
        return 0.0

    # 属于某一分类的内容项数量
    def catcount(self, cat):
        if cat in self.cc:
            return float(self.cc[cat])
        return 0

    # 所有内容项的数量
    def totalcount(self):
        return sum(self.cc.values())

    # 所有分类的列表
    def categories(self):
        return self.cc.keys()

    def train(self, item, cat):
        features = self.getfeatures(item)
        # 针对该分类为每个特征增加计数值
        for f in features:
            self.incf(f, cat)

        # 增加针对该分类的计数值
        self.incc(cat)


def main():
    cl = classifier(getwords)
    cl.train('the quick brown for jumps over the lazy dog', 'good')
    cl.train('make quick money in the online casino', 'bad')
    print(cl.fcount('quick', 'good'))
    print(cl.fcount('quick', 'bad'))

if __name__ == '__main__':
    main()
