package com.mine.ljvm.gc;

/* VM options: -verbose:gc -XX:+PrintGCDetails  */

public class ReferenceCountingGC {
    private static final int _MB = 1024 * 1024;
    public Object instance = null;
    private byte[] bigSize = new byte[2 * _MB];

    public static void testGC() {
        ReferenceCountingGC objA = new ReferenceCountingGC();
        ReferenceCountingGC objB = new ReferenceCountingGC();
        objA.instance = objB;
        objB.instance = objA;
        objA = null;
        objB = null;
        System.gc();
    }

    public static void main(String[] args) {
        ReferenceCountingGC.testGC();
    }
}
