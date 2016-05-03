# learn-jvm
notes and code with learning jvm

## important

Some program in src are very harmful, it will teardown your pc.
so should run them with cautious.

Some special java VM options were put on the top of the files in src.

## java vm args examples

* ``-verbose:gc -XX:+PrintGCDetails``
    print gc details
* ``java -XX:+PrintFlagsFinal -version| grep xxx``
    print all available options

## important topic

* remove PermGen, add Metaspace in JDK8
    https://dzone.com/articles/java-8-permgen-metaspace
    http://openjdk.java.net/jeps/122

* gc about
    * reference counting
    * reachability analysis
    * copy in Young
    * make-sweep in Tenured
    * Serial/ParNew in Young
    * Parallel Scavenge in Young, point on throughput
        (MAXGCPauseMills/GCTimeRatio/UseAdaptiveSizePolicy)
    * Serial Old in Tenured(with Serial in Young)
    * Parallel Old in Tenured(with Parallel Scavenge in Young)
    * CMS(Concurrent Mark Sweep) focus on (min GC pause time)
    * G1(Garbage-First)

* jdk tools
	* jps
	* jstat
	* jinfo
	* jmap(memory map for Java)
	* jhat(JVM Heap Analysis Tool)
	* jstack(Stack Track for Java)
