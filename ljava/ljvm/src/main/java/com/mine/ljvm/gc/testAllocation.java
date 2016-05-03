package com.mine.ljvm.gc;

/* VM args: -verbose:gc -Xms20M -Xmx20M -Xmn10M -XX:+PrintGCDetails -XX:SurvivorRatio=8 */

public class testAllocation {
    private static final int _1M = 1024 * 1024;

    public static void main(String[] args) {
        testAllocation.testAllocateEden();
    }

    private static void testAllocateEden() {
        byte[] blist1, blist2, blist3, blist4;
        blist1 = new byte[2 * _1M];
        blist2 = new byte[2 * _1M];
        blist3 = new byte[2 * _1M];
        blist4 = new byte[4 * _1M];
        System.out.println(blist1.length + blist2.length + blist3.length + blist4.length);
    }
}
