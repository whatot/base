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
        # 最小阀值，归类倍数
        self.thresholds = {}

    def setthreshold(self, cat, t):
        self.thresholds[cat] = t

    def getthreshold(self, cat):
        if cat not in self.thresholds:
            return 1.0
        return self.thresholds[cat]

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

    def fprob(self, f, cat):
        if self.catcount(cat) == 0:
            return 0
        # 特征在分类中出现的总次数，除以分类中包含内容项的总数
        return self.fcount(f, cat) / self.catcount(cat)

    def weightedprob(self, f, cat, prf, weight=1.0, ap=0.5):
        # 计算当前的概率值
        basicprob = prf(f, cat)

        # 统计特征在所有分类中出现的次数
        totals = sum([self.fcount(f, c) for c in self.categories()])

        # 计算加权平均
        bp = ((weight*ap) + (totals*basicprob)) / (weight+totals)
        return bp

    def classify(self, item, default=None):
        probs = {}
        # 寻找概率最大的分类
        max = 0.0
        for cat in self.categories():
            probs[cat] = self.prob(item, cat)
            if probs[cat] > max:
                max = probs[cat]
                best = cat
        # 确保概率值超出 域值*次大概率值
        # 可能属于的，但没有超过的归于default类
        for cat in probs:
            if cat == best:
                continue
            if probs[cat]*self.getthreshold(best) > probs[best]:
                return default
        return best


class naivebayes(classifier):
    def docprob(self, item, cat):
        features = self.getfeatures(item)

        # 将所有特征的概率相乘
        p = 1
        for f in features:
            p *= self.weightedprob(f, cat, self.fprob)
        return p

    # Pr(Category | Document) =
    #           Pr(Document | Category) x Pr(Category)/Pr(Document)
    def prob(self, item, cat):
        catprob = self.catcount(cat) / self.totalcount()
        docprob = self.docprob(item, cat)
        return docprob * catprob


# 1.0
# 1.0
def testSimple():
    cl = classifier(getwords)
    cl.train('the quick brown for jumps over the lazy dog', 'good')
    cl.train('make quick money in the online casino', 'bad')
    print(cl.fcount('quick', 'good'))
    print(cl.fcount('quick', 'bad'))


# 0.666666666667
def testFprob():
    cl = classifier(getwords)
    simpletrain(cl)
    print(cl.fprob('quick', 'good'))


# 0.25
# 0.166666666667
def testWeightedprob():
    cl = classifier(getwords)
    simpletrain(cl)
    print(cl.weightedprob('money', 'good', cl.fprob))
    simpletrain(cl)
    print(cl.weightedprob('money', 'good', cl.fprob))


# 0.15625
# 0.05
def testnaivebayes():
    cl = naivebayes(getwords)
    simpletrain(cl)
    print(cl.prob('quick rabbit', 'good'))
    print(cl.prob('quick rabbit', 'bad'))


# good
# bad
# unknown
# bad
def testClassify():
    cl = naivebayes(getwords)
    simpletrain(cl)
    print(cl.classify('quick rabbit', default='unknown'))
    print(cl.classify('quick money', default='unknown'))
    cl.setthreshold('bad', 3.0)
    print(cl.classify('quick money', default='unknown'))
    for i in range(10):
        simpletrain(cl)
    print(cl.classify('quick money', default='unknown'))


def simpletrain(cl):
    cl.train('Nobody owns the water.', 'good')
    cl.train('the quick rabbit jumps fences', 'good')
    cl.train('buy pharmaceuticals now', 'bad')
    cl.train('make quick money at the online casino', 'bad')
    cl.train('the quick brown fox jumps', 'good')


if __name__ == '__main__':
    testClassify()
