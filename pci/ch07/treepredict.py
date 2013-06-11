#!/usr/bin/python2
# -*- coding:utf-8 -*-

# wget http://kiwitobes.com/tree/decision_tree_example.txt
with open('decision_tree_example.txt', 'r') as tree:
    my_data = [line.split('\t') for line in tree]
# print(my_data)


# 对各种可能的结果进行计数(每一行数据的最后一列记录了这一计数结果)
def uniquecounts(rows):
    results = {}
    for row in rows:
        # 计数结果在最后一列
        r = row[len(row) - 1]
        if r not in results:
            results[r] = 0
        results[r] += 1
    return results


class decisionnode:
    def __init__(self, col=-1, value=None, results=None, tb=None, fb=None):
        # col 是待检验的判断条件所对应的列索引值
        # value 对应于为了使结果为true，当前列必须匹配的值
        # results 保存的是针对于当前分支的结果，是一个dict。
        #         除叶节点外，在其他节点上该值都为None
        # tb, fb 也是decisionnode，他们对应结果分别为true或false时，
        #       树上相对于当前节点的子树上的节点
        self.col = col
        self.value = value
        self.results = results
        self.tb = tb
        self.fb = fb

    # 在某一列上对数据集合进行拆分，能够处理数值型数据或名词性数据。
    def divideset(self, rows, column, value):
        # 定义一个函数，令其告诉我们数据行属于第一组(true)还是第二组(false)
        split_function = None
        if isinstance(value, int) or isinstance(value, float):
            split_function = lambda row: row[column] >= value
        else:
            split_function = lambda row: row[column] == value

        # 将数据集拆分成两个几何，并返回
        set1 = [row for row in rows if split_function(row)]
        set2 = [row for row in rows if not split_function(row)]
        return (set1, set2)

    # 随机放置的数据项出现于错误分类中的概率
    def giniimpurify(self, rows):
        total = len(rows)
        counts = uniquecounts(rows)
        imp = 0
        for k1 in counts:
            p1 = float(counts[k1]) / total
            for k2 in counts:
                if k1 == k2:
                    continue
                p2 = float(counts[k2]) / total
                imp += p1 * p2
        return imp

    # 熵是遍历所有可能的结果之后所得到的 p(x) * log(p(x)) 之和
    # p(i) = frequency(outcome) = count(outcome) / count(total rows)
    # Entropy = 针对所有结果的 p(i) * log(p(i)) 之和
    def entropy(self, rows):
        from math import log
        log2 = lambda x: log(x) / log(2)
        results = uniquecounts(rows)
        # 此处开始计算熵的值
        ent = 0.0
        for r in results.keys():
            p = float(results[r]) / len(rows)
            ent = ent - p*log2(p)
        return ent


def test_divideset():
    tp = decisionnode()
    print(tp.divideset(my_data, 2, 'yes'))


# tp.giniimpurify(my_data)
# 0.671875
# tp.entropy(my_data)
# 1.74899922306
# tp.entropy(set1)
# 1.75
# tp.giniimpurify(set1)
# 0.65625
def test_gini_entropy():
    tp = decisionnode()
    print('tp.giniimpurify(my_data)')
    print(tp.giniimpurify(my_data))
    print('tp.entropy(my_data)')
    print(tp.entropy(my_data))
    set1, set2 = tp.divideset(my_data, 2, 'yes')
    print('tp.entropy(set1)')
    print(tp.entropy(set1))
    print('tp.giniimpurify(set1)')
    print(tp.giniimpurify(set1))

if __name__ == '__main__':
    test_gini_entropy()
