# Ch03-text

####subprocess.Popen; subprocess.PIPE; in, not in; index(); find();

    In [3]: import subprocess
    In [4]: res = subprocess.Popen(['uname', '-sv'], stdout=subprocess.PIPE)
    In [5]: uname = res.stdout.read().strip()
    In [6]: uname
    Out[6]: 'Linux #1 SMP Debian 3.2.41-2'
    In [7]: 'Linux' in uname
    Out[7]: True
    In [8]: 'Linux' not in uname
    Out[8]: False
    In [9]: uname.index('Debian')
    Out[9]: 13
    In [10]: uname.find('Linux')
    Out[10]: 0
    In [11]: uname.find('inux')
    Out[11]: 1
    In [12]: uname.find('linux')
    Out[12]: -1
    In [13]: uname.index('linux')
    ...
    ValueError: substring not found


####字符串切分, 返回一个新的字符串对象

    In [22]: smp_index = uname.index('SMP')
    In [23]: smp_index
    Out[23]: 9
    In [24]: uname[smp_index:]
    Out[24]: 'SMP Debian 3.2.41-2'


##### stratswith(); endswith()

    In [26]: uname.startswith('Li')
    Out[26]: True
    In [27]: uname
    Out[27]: 'Linux #1 SMP Debian 3.2.41-2'
    In [28]: uname.endswith(2)
    TypeError: endswith first arg must be str, unicode, or tuple, not int
    In [29]: uname.endswith('2')
    Out[29]: True


#### strip(); lstrip(); rstrip() 去除空白

    In [1]: spas = "\t\n some no-space--- text \n \t\n"
    In [2]: spas
    Out[2]: '\t\n some no-space--- text \n \t\n'
    In [3]: print spas
        
    some no-space--- text 


    In [4]: spas.lstrip()
    Out[4]: 'some no-space--- text \n \t\n'
    In [5]: spas.strip()
    Out[5]: 'some no-space--- text'
    In [6]: spas.rstrip()
    Out[6]: '\t\n some no-space--- text'
    In [8]: spas.strip().strip("so-pt")
    Out[8]: 'me no-space--- tex'


#### upper(); lower()

#### split(), 定界字符串, 默认以空格分割

    strings.split(',')
    strings.split(',', 1)  指定分割的最多次数

#### splitlines()返回一个字符串的每一行所组成的列表，并且保存为一组。

#### join() 输入为str, 非整数or others

    In [9]: some_join = ['aaa', 'bbb', 'ccc', 'ddd']
    In [10]: ', '.join(some_join)
    Out[10]: 'aaa, bbb, ccc, ddd'
    In [11]: ''.join(some_join)
    Out[11]: 'aaabbbcccddd'
    In [12]: '\t'.join(some_join)
    Out[12]: 'aaa\tbbb\tccc\tddd'

    In [13]: some_list = range(10)
    In [14]: some_list
    Out[14]: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    In [15]: ",".join([str(i) for i in some_list])
    Out[15]: '0,1,2,3,4,5,6,7,8,9'


#### replace()

    In [16]: ''.join(some_join).replace("a", "b")
    Out[16]: 'bbbbbbcccddd'


#### unicode

    In [17]: unicode_string = u'jcjdchiduf'
    In [18]: unicode_string
    Out[18]: u'jcjdchiduf'
    In [20]: unicode('unicode_string')
    Out[20]: u'unicode_string'
    In [21]: unicode_string = u'abc_\u03a0\u03a3\u03a9_\u0414\u0424\u042F'
    In [22]: unicode_string
    Out[22]: u'abc_\u03a0\u03a3\u03a9_\u0414\u0424\u042f'
    In [23]: print unicode_string.encode('utf-8')
    abc_ΠΣΩ_ДФЯ


#### re


