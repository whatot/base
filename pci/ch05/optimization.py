#!/usr/bin/python2
# -*- coding:utf-8

import time
import random
import math


def getminutes(t):
    x = time.strptime(t, '%H:%M')
    return x[3] * 60 + x[4]


class scheduleoptization:
    def __init__(self):
        self.people = [('Seymour', 'BOS'),
                       ('Franny', 'DAL'),
                       ('Zooey', 'CAK'),
                       ('Walt', 'MIA'),
                       ('Buddy', 'ORD'),
                       ('Les', 'OMA')]

        # New York 的 LaGuardia 机场
        self.destn = 'LGA'
        self.flights = {}

        with open('schedule.txt', 'r') as file:
            for line in file:
                origin, dest, depart, arrive, price = line.strip().split(',')
                self.flights.setdefault((origin, dest), [])
                # 将航班详情添加到航班列表中
                self.flights[(origin, dest)].append((depart, arrive,
                                                     int(price)))

    def printschedule(self, r):
        for d in range(len(r)/2):
            name = self.people[d][0]
            origin = self.people[d][1]
            out = self.flights[(origin, self.destn)][r[2 * d]]
            ret = self.flights[(self.destn, origin)][r[2 * d + 1]]
            print('%10s%10s %5s-%5s $%3s %5s-%5s $%3s'
                  % (name, origin, out[0], out[1], out[2],
                     ret[0], ret[1], ret[2]))

    def schedulecost(self, sol):
        totalprice = 0
        latestarrival = 0
        earliestdep = 24 * 60

        for d in range(len(sol) / 2):
            # 得到往程航班和返程航班
            origin = self.people[d][1]
            outbound = self.flights[(origin, self.destn)][int(sol[2 * d])]
            returnf = self.flights[(self.destn, origin)][int(sol[2 * d + 1])]

            # 总价格等于所有往程航班和返程航班价格之和
            totalprice += outbound[2]
            totalprice += returnf[2]

            # 记录最晚到达时间和最早离开时间
            if latestarrival < getminutes(outbound[1]):
                latestarrival = getminutes(outbound[1])
            if earliestdep > getminutes(returnf[0]):
                earliestdep = getminutes(returnf[0])

        # 每个人必须在机场等待直到最后一个人到达为止
        # 他们也必须在相同时间到达， 并等候他们的返程航班
        totalwait = 0
        for d in range(len(sol) / 2):
            origin = self.people[d][1]
            outbound = self.flights[(origin, self.destn)][int(sol[2 * d])]
            returnf = self.flights[(self.destn, origin)][int(sol[2 * d + 1])]
            totalwait += latestarrival - getminutes(outbound[1])
            totalwait += getminutes(returnf[0]) - earliestdep

        # 汽车租用费用， $50
        if latestarrival < earliestdep:
            totalprice += 50

        return totalprice + totalwait

    def randomoptimize(self, domain, costf):
        best = 999999
        bestr = None

        for i in range(1000):
            # 创建一个随机解
            r = [random.randint(domain[i][0], domain[i][1])
                 for i in range(len(domain))]
            # 得到成本
            cost = costf(r)

            # 与到目前为止的最优解进行比较
            if cost < best:
                best = cost
                bestr = r

        return bestr

    def hillclimb(self, domain, costf):
        # 创建一个随机解
        sol = [random.randint(domain[i][0], domain[i][1])
               for i in range(len(domain))]

        # 主循环
        while 1:
            # 创建相邻解的列表
            neighbors = []
            for j in range(len(domain)):
                # 在每个方向上相对于原值偏移一点
                if sol[j] > domain[j][0]:
                    neighbors.append(sol[0:j] + [sol[j] - 1] + sol[j+1:])
                if sol[j] < domain[j][0]:
                    neighbors.append(sol[0:j] + [sol[j] + 1] + sol[j+1:])

            # 在相邻解中寻找最优解
            current = costf(sol)
            best = current
            for j in range(len(neighbors)):
                cost = costf(neighbors[j])
                if cost < best:
                    best = cost
                    sol = neighbors[j]

            # 如果没有更好的解， 则退出循环
            if best == current:
                break

        return sol


def testschedule(scheduleoptization):
    schopti = scheduleoptization()
    s = [1, 4, 3, 2, 7, 3, 6, 3, 2, 4, 5, 3]
    schopti.printschedule(s)
    print(schopti.schedulecost(s))


def testrandomoptimize():
    schopti = scheduleoptization()
    domain = [(0, 9)] * (len(schopti.people) * 2)
    s = schopti.randomoptimize(domain, schopti.schedulecost)
    schopti.printschedule(s)
    print(schopti.schedulecost(s))


def testhillclimb():
    schopti = scheduleoptization()
    domain = [(0, 9)] * (len(schopti.people) * 2)
    s = schopti.hillclimb(domain, schopti.schedulecost)
    schopti.printschedule(s)
    print(schopti.schedulecost(s))


if __name__ == '__main__':
    testhillclimb()
