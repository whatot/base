#!/usr/bin/python2
# -*- coding:utf-8 -*-

import math
from PIL import Image, ImageDraw
import optimization


people = ['Charlie', 'Augustus', 'Veruca', 'Violet', 'Mike', 'Joe', 'Willy',
          'Miranda']

links = [('Augustus', 'Willy'),
         ('Mike', 'Joe'),
         ('Miranda', 'Mike'),
         ('Violet', 'Augustus'),
         ('Miranda', 'Willy'),
         ('Charlie', 'Mike'),
         ('Veruca', 'Joe'),
         ('Miranda', 'Augustus'),
         ('Willy', 'Augustus'),
         ('Joe', 'Charlie'),
         ('Veruca', 'Augustus'),
         ('Miranda', 'Joe')]

domain = [(10, 370)] * (len(people) * 2)

sol = [120, 200, 250, 125, 100, 320, 220, 130,
       210, 240, 250, 310, 118, 129, 223, 334]


def crosscount(v):
    # 将数字序列转换成一个 pearson: (x,y) 的字典
    loc = dict([(people[i], (v[i*2], v[i*2+1]))
                for i in range(0, len(people))])
    total = 0

    # 遍历每一对连线
    for i in range(len(links)):
        for j in range(i+1, len(links)):

            # 获取坐标位置
            (x1, y1), (x2, y2) = loc[links[i][0]], loc[links[i][1]]
            (x3, y3), (x4, y4) = loc[links[j][0]], loc[links[j][1]]

            den = (y4-y3)*(x2-x1) - (x4-x3)*(y2-y1)

            # 如果两线平行，则 den == 0
            if den == 0:
                continue

            # 否则，ua 与 ub 就是两条交叉线的分数值
            ua = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / den
            ub = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / den

            # 如果两条线的分数介于 0 和 1 之间，则两线彼此交叉
            if ua > 0 and ua < 1 and ub > 0 and ub < 1:
                total += 1

    for i in range(len(people)):
        for j in range(i+1, len(people)):
            # 获得连接点的位置
            (x1, y1), (x2, y2) = loc[people[i]], loc[people[j]]

            # 计算两结点的间距
            dist = math.sqrt(math.pow(x1-x1, 2) + math.pow(y1-y2, 2))
            # 对间距小于 50 个像素的结点进行判别
            if dist < 50:
                total += (1.0 - (dist/50.0))

    return total


def drawnetwork(sol):
    # 建立 image 对象
    img = Image.new('RGB', (400, 400), (255, 255, 255))
    draw = ImageDraw.Draw(img)

    # 建立标志位置信息的字典
    pos = dict([(people[i], (sol[i*2], sol[i*2+1]))
                for i in range(0, len(people))])

    # 绘制连线
    for (a, b) in links:
        draw.line((pos[a], pos[b]), fill=(255, 0, 0))

    # 绘制代表人的结点
    for n, p in pos.items():
        draw.text(p, n, (0, 0, 0))

    img.show()


def main():
    opti = optimization.scheduleoptization()

    # print('randomoptimize')
    # sol = opti.randomoptimize(domain, crosscount)

    # print('annealingoptimize')
    # sol = opti.annealingoptimize(domain, crosscount)

    print('geneticoptimize')
    sol = opti.geneticoptimize(domain, crosscount)

    print(crosscount(sol))
    print(sol)
    drawnetwork(sol)

if __name__ == '__main__':
    main()
