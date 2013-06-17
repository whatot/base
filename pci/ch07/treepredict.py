#!/usr/bin/python2
# -*- coding:utf-8 -*-
from PIL import Image, ImageDraw

# wget http://kiwitobes.com/tree/decision_tree_example.txt
with open('decision_tree_example.txt', 'r') as tree:
    my_data = [line.split('\t') for line in tree]
# print(my_data)


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
def divideset(rows, column, value):
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


# 随机放置的数据项出现于错误分类中的概率
def giniimpurify(rows):
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
def entropy(rows):
    from math import log
    log2 = lambda x: log(x) / log(2)
    results = uniquecounts(rows)
    # 此处开始计算熵的值
    ent = 0.0
    for r in results.keys():
        p = float(results[r]) / len(rows)
        ent = ent - p*log2(p)
    return ent


def buildtree(rows, scoref=entropy):
    if len(rows) == 0:
        return decisionnode()
    current_score = scoref(rows)

    # 定义一些变量以记录最佳拆分条件
    best_gain = 0.0
    best_criteria = None
    best_sets = None

    column_count = len(rows[0]) - 1
    for col in range(0, column_count):
        # 在当前列中生成一个由不同值构成的序列
        column_values = {}
        for row in rows:
            column_values[row[col]] = 1
        # 接下来根据这一列中的每个值，尝试对数据集进行拆分
        for value in column_values.keys():
            (set1, set2) = divideset(rows, col, value)

            # 信息增益
            p = float(len(set1)) / len(rows)
            gain = current_score - p*scoref(set1) - (1-p)*scoref(set2)
            if gain > best_gain and len(set1) > 0 and len(set2) > 0:
                best_gain = gain
                best_criteria = (col, value)
                best_sets = (set1, set2)
    # 创建子分支
    if best_gain > 0:
        trueBranch = buildtree(best_sets[0])
        falseBranch = buildtree(best_sets[1])
        return decisionnode(col=best_criteria[0], value=best_criteria[1],
                            tb=trueBranch, fb=falseBranch)
    else:
        return decisionnode(results=uniquecounts(rows))


def printtree(tree, indent=' '):
    # 这是一个叶节点吗？
    if tree.results is not None:
        print(str(tree.results))
    else:
        # 打印判断条件
        print(str(tree.col) + ':' + str(tree.value) + '? ')
        # 打印分支
        print(indent + 'T->'),
        printtree(tree.tb, indent + '  ')
        print(indent + 'F->'),
        printtree(tree.fb, indent + '  ')


def getwidth(tree):
    if tree.tb is None and tree.fb is None:
        return 1
    return getwidth(tree.tb) + getwidth(tree.fb)


def getdepth(tree):
    if tree.tb is None and tree.fb is None:
        return 0
    return max(getdepth(tree.tb), getdepth(tree.fb)) + 1


def drawtree(tree, jpeg='tree.jpg'):
    w = getwidth(tree) * 100
    h = getdepth(tree) * 100 + 120

    img = Image.new('RGB', (w, h), (255, 255, 255))
    draw = ImageDraw.Draw(img)

    drawnode(draw, tree, w/2, 20)
    img.save(jpeg, 'JPEG')


def drawnode(draw, tree, x, y):
    if tree.results is None:
        # 得到每个分支的宽度
        w1 = getwidth(tree.fb) * 100
        w2 = getdepth(tree.tb) * 100

        # 确定此节点所要占据的总空间
        left = x - (w1 + w2) / 2
        right = x + (w1 + w2) / 2

        # 绘制判断条件字符串
        draw.text((x-20, y-10), str(tree.col)+':'+str(tree.value), (0, 0, 0))

        # 绘制到分支的连线
        draw.line((x, y, left+w1/2, y+100), fill=(255, 0, 0))
        draw.line((x, y, right-w2/2, y+100), fill=(255, 0, 0))

        # 绘制分支的节点
        drawnode(draw, tree.fb, left+w1/2, y+100)
        drawnode(draw, tree.tb, right-w2/2, y+100)
    else:
        txt = ' \n'.join(['%s:%d' % v for v in tree.results.items()])
        draw.text((x-20, y), txt, (0, 0, 0))


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


'''
$python2 --
0:google?
 T-> 3:18?
   T-> 2:yes?
     T-> {'Basic\n': 1}
     F-> {'None\n': 1}
   F-> {'Premium\n': 3}
 F-> 3:19?
   T-> 0:kiwitobes?
     T-> 1:UK?
       T-> {'None\n': 1}
       F-> {'Basic': 1}
     F-> {'None\n': 1}
   F-> 0:slashdot?
     T-> {'None\n': 2}
     F-> 2:yes?
       T-> {'Basic\n': 3}
       F-> 1:UK?
         T-> {'Basic\n': 1}
         F-> {'None\n': 2}
'''


def testprinttree():
    tree = buildtree(my_data)
    printtree(tree)


def testdrawtree():
    tree = buildtree(my_data)
    drawtree(tree, jpeg='treeview.jpg')


if __name__ == '__main__':
    testprinttree()
    testdrawtree()
