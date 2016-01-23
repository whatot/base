# -*- coding:utf-8 -*-

# 1. simplest one
def fab1(max):
    n, a, b = 0, 0, 1
    while n < max:
        print b
        a, b = b, a + b
        n = n + 1


# 2. 返回一个list
def fab2(max):
    n, a, b = 0, 0, 1
    L = []
    while n < max:
        L.append(b)
        a, b = b, a + b
        m = n + 1
    return L

"""
可以通过
for n in fab2(5):
    print n
打印返回的list
NOTE：函数在运行中所占用的内存会随参数max的增大而增大
如果要控制内存占用，最好不要用list
"""


"""
通过iterable对象来迭代

for i in range(1000); pass
会生成一个1000个元素的list

for i in xrange(1000); pass
不会生成一个1000个元素的list，而是在每次迭代中返回下一个数值，内存占用很小。
因为xrange不返回list，而是返回一个iterable对象。
"""

# 3. iterable
class fab3(object):

    def __init__(self, max):
        self.max = max
        self.n, self.a, self.b = 0, 0, 1

    def __iter__(self):
        return self

    def next(self):
        if self.n < self.max:
            r = self.b
            self.a, self.b = self.b, self.a + self.b
            self.n = self.n + 1
            return r
        raise StopIteration()

"""
for n in fab3(5):
    print n
"""


# 4 .yield --> iterable
def fab4(max):
    n, a, b = 0, 0, 1
    while n < max:
        yield b
        #print n
        a, b = b, a+b
        n = n + 1

"""
yield的作用就是把一个函数变成一个generator对象，该对象具有next(方法)

 >>> f = fab(5)
 >>> f.next()
 1
 >>> f.next()
 1
 >>> f.next()
 2
 >>> f.next()
 3
 >>> f.next()
 5
 >>> f.next()
 Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
 StopIteration


当函数执行结束时，generator 自动抛出 StopIteration 异常，表示迭代完成。在 for 循环里，无需处理 StopIteration 异常，循环会正常结束。

我们可以得出以下结论：

一个带有 yield 的函数就是一个 generator，它和普通函数不同，生成一个 generator 看起来像函数调用，但不会执行任何函数代码，直到对其调用 next()（在 for 循环中会自动调用 next()）才开始执行。虽然执行流程仍按函数的流程执行，但每执行到一个 yield 语句就会中断，并返回一个迭代值，下次执行时从 yield 的下一个语句继续执行。看起来就好像一个函数在正常执行的过程中被 yield 中断了数次，每次中断都会通过 yield 返回当前的迭代值。

yield 的好处是显而易见的，把一个函数改写为一个 generator 就获得了迭代能力，比起用类的实例保存状态来计算下一个 next() 的值，不仅代码简洁，而且执行流程异常清晰。

如何判断一个函数是否是一个特殊的 generator 函数？可以利用 isgeneratorfunction 判断：


清单 7. 使用 isgeneratorfunction 判断

 >>> from inspect import isgeneratorfunction
 >>> isgeneratorfunction(fab)
 True


要注意区分 fab 和 fab(5)，fab 是一个 generator function，而 fab(5) 是调用 fab 返回的一个 generator，好比类的定义和类的实例的区别：

清单 8. 类的定义和类的实例


 >>> import types
 >>> isinstance(fab, types.GeneratorType)
 False
 >>> isinstance(fab(5), types.GeneratorType)
 True


fab 是无法迭代的，而 fab(5) 是可迭代的：

 >>> from collections import Iterable
 >>> isinstance(fab, Iterable)
 False
 >>> isinstance(fab(5), Iterable)
 True


每次调用 fab 函数都会生成一个新的 generator 实例，各实例互不影响：

 >>> f1 = fab(3)
 >>> f2 = fab(5)
 >>> print 'f1:', f1.next()
 f1: 1
 >>> print 'f2:', f2.next()
 f2: 1
 >>> print 'f1:', f1.next()
 f1: 1
 >>> print 'f2:', f2.next()
 f2: 1
 >>> print 'f1:', f1.next()
 f1: 2
 >>> print 'f2:', f2.next()
 f2: 2
 >>> print 'f2:', f2.next()
 f2: 3
 >>> print 'f2:', f2.next()
 f2: 5


回页首

return 的作用

在一个 generator function 中，如果没有 return，则默认执行至函数完毕，如果在执行过程中 return，则直接抛出 StopIteration 终止迭代。

回页首

另一个例子

另一个 yield 的例子来源于文件读取。如果直接对文件对象调用 read() 方法，会导致不可预测的内存占用。好的方法是利用固定长度的缓冲区来不断读取文件内容。通过 yield，我们不再需要编写读文件的迭代类，就可以轻松实现文件读取：

清单 9. 另一个 yield 的例子
"""

def read_file(fpath):
    BLOCK_SIZE = 1024
    with open(fpath, 'rb') as f:
        while True:
            block = f.read(BLOCK_SIZE)
            if block:
                yield block
            else:
                return

"""
以上仅仅简单介绍了 yield 的基本概念和用法，yield 在 Python 3 中还有更强大的用法，我们会在后续文章中讨论。

注：本文的代码均在 Python 2.7 中调试通过

"""

