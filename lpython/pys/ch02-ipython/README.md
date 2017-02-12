# Ch02-Ipython notes


1. 输出的差异：为变量赋值，显示对变量求值的结果，打印输出变量


    In [19]: a=1

    In [20]: a
    Out[20]: 1

    In [21]: print a
    1


2. In与Out的差异


    In [24]: type(Out)
    Out[24]: dict

    In [25]: type(In)
    Out[25]: list

    print In
    print Out


3. 配置IPython

4. 魔力函数(magic %)

    In [26]: %lsmagic
    Available line magics:
    %alias  %alias_magic  %autocall  %autoindent  %automagic  %bookmark  %cd  %colors  %config  %cpaste  %debug  %dhist  %dirs  %doctest_mode  %ed  %edit  %env  %gui  %hist  %history  %install_default_config  %install_ext  %install_profiles  %killbgscripts  %load  %load_ext  %loadpy  %logoff  %logon  %logstart  %logstate  %logstop  %lsmagic  %macro  %magic  %notebook  %page  %paste  %pastebin  %pdb  %pdef  %pdoc  %pfile  %pinfo  %pinfo2  %popd  %pprint  %precision  %profile  %prun  %psearch  %psource  %pushd  %pwd  %pycat  %pylab  %quickref  %recall  %rehashx  %reload_ext  %rep  %rerun  %reset  %reset_selective  %run  %save  %sc  %store  %sx  %system  %tb  %time  %timeit  %unalias  %unload_ext  %who  %who_ls  %whos  %xdel  %xmode

    Available cell magics:
    %%!  %%bash  %%capture  %%file  %%perl  %%prun  %%python3  %%ruby  %%script  %%sh  %%sx  %%system  %%timeit

    Automagic is ON, % prefix IS NOT needed for line magics.


帮助信息

    In [1]: %magic 

    In [2]: something ?

    In [3]: %quickref ?


5. UNIX Shell

### alias

**do-nothing方法**, 无附加参数

**do-everything方法** , 隐含参数


    In [28]: %alias nss netstat -lptn

    In [29]: nss
    (Not all processes could be identified, non-owned process info
    will not be shown, you would have to be root to see it all.)
    Active Internet connections (only servers)
    Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name
    tcp        0      0 0.0.0.0:50494           0.0.0.0:*               LISTEN      31275/transmission-
    tcp6       0      0 :::50494                :::*                    LISTEN      31275/transmission-

    In [30]: nss |grep  0.0.0.0
    (Not all processes could be identified, non-owned process info
    will not be shown, you would have to be root to see it all.)
    tcp        0      0 0.0.0.0:50494           0.0.0.0:*               LISTEN      31275/transmission-



    In [31]: %alias echoo echo first: "|%s|", second: "|%s|"

    In [32]: echoo foo bat
    first: |foo|, second: |bat|

    In [35]: echoo foo bat ecm
    first: |foo|, second: |bat| ecm

    In [36]: echoo foo
    ERROR: Alias <echoo> requires 2 arguments, 1 given.



    In [40]: store echoo  保留别名


### Shell的执行


    In [37]: !netstat -lptn
    (Not all processes could be identified, non-owned process info
    will not be shown, you would have to be root to see it all.)
    Active Internet connections (only servers)
    Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name
    tcp        0      0 0.0.0.0:50494           0.0.0.0:*               LISTEN      31275/transmission-
    tcp6       0      0 :::50494                :::*                    LISTEN      31275/transmission-

    In [38]: user = 'root'

    In [39]: process = 'init'

    In [40]: !ps aux |grep $user | grep $process
    root         1  0.0  0.0   2280   512 ?        Ss    4月09   0:08 init [2]

    In [41]: l = !ps aux |grep $user | grep $process

    In [42]: l
    Out[42]: ['root         1  0.0  0.0   2280   512 ?        Ss    4\xe6\x9c\x8809   0:08 init [2]  ']

    In [43]: type(l)
    Out[43]: IPython.utils.text.SList


!!可以替换!, 除了使用!无法保存结果到变量之外，两者完全一致。

#### rehash

#### rehashx

#### cd


    cd              回到主目录
    cd -            回到前一个目录
    cd -q           (quiet)不输出刚刚改变的目录
    cd -5           历史目录列表
    cd -b bookmark_name


#### bookmark


    cd /tmp
    bookmark tm
    bookmark b ~/static/base/
    bookmark -l
    bookmark -r                     删除全部标签
    bookmark -d bookmark_name       删除该标签


#### dhist 历史目录列表

    cd -<TAB>
    dhist 5 最近访问过的5个目录
    dhist 3 7 查看第3个到第6个之间的所有目录

#### pwd

#### 可变扩展


    In [1]: for i in range(10): !date > {i}.txt

    In [2]: ls
    0.txt  3.txt  6.txt  9.txt           new_pysysinfo.py   pysysinfo.py
    1.txt  4.txt  7.txt  bashsysinfo.sh  new_pysysinfo.pyc  pysysinfo.pyc
    2.txt  5.txt  8.txt  man_pipe.c      ping-ip.py

    In [3]: !cat 0.
    0.bit_length   0.denominator  0.numerator    0.txt
    0.conjugate    0.imag         0.real

    In [3]: !cat 0.txt
    2013年 04月 17日 星期三 21:05:02 CST


#### 字符串处理


    In [10]: ps = !ps aux

    In [11]: ps.grep('conky')
    Out[11]: ['whatot    3907  0.8  0.0  80376  2060 ?        Sl    4\xe6\x9c\x8809 106:34 conky --quiet']


反向选择

    In [34]: ps.grep('[abcde]', prune=True)
    Out[34]: 
    ['root         1  0.0  0.0   2280   512 ?        Ss    4\xe6\x9c\x8809   0:08 init [2]  ',
    'root      1933  0.0  0.0      0     0 ?        S<    4\xe6\x9c\x8809   0:00 [iprt]',
    'root     15596  0.0  0.0      0     0 ?        S    18:28   0:00 [flush-8:16]',
    'root     29848  0.0  0.0      0     0 ?        S     4\xe6\x9c\x8812   0:11 [flush-8:0]']

    In [35]: ps.grep('[abcde]', prune=True).fields(0,1,2,3)
    Out[35]: 
    ['root 1 0.0 0.0',
    'root 1933 0.0 0.0',
    'root 15596 0.0 0.0',
    'root 29848 0.0 0.0']

``ps aux |awk '{if ($1 == "root") print $2}'``

``ps.grep('root', field=0).fields(1)``


#### sh profile


6. 信息搜集

#### page

#### pdef 打印任何被调用对象的定义名或是函数声明

#### pdoc 打印传递给它的函数的注释信息

#### pfile 能够运行对象的文件

#### pinfo 等同于  f ? 或者 ? f (f 为对象名); 以及更详细的??

#### psource 显示定义的元素的源代码

#### psearch 据名称或同配符查找python对象

#### who 列出所有交互式对象的方法; who_ls 返回的是一个列表; whos输出详细信息


### 历史

    C-a         回到行首
    C-e         跳到行尾
    C-f         删除字符
    C-h         向后删除字符(like backspace)
    C-p         将历史记录中的行向后移动一行
    C-nt        将历史记录中的行向前移动一行
    man readline

#### hist
    
    hist -n             去行号
    hist -t             输出所有
    hist -r             准确显示输入了什么
    hist -g hist        搜索指定模式

#### 历史输出

    _                   显示上次输出
    _1, _2, _3, ...     对应Out[1], Out[2], Out[3].....


7. 自动与快捷方式

#### alias

#### macro

#### store

#### reset

#### run 执行指定文件

#### save 保存指定的输入行到输出文件 -r

    save options filename lines 

#### rep 自动启用函数
