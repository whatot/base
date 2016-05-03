package com.mine.ljvm.oom;

/* VM options: -Xss232k */

public class JavaVMStackSOF {
    private int stackLength = 1;

    public static void main(String[] args) {
        JavaVMStackSOF oom = new JavaVMStackSOF();
        try {
            oom.stackLeak();
        } catch (Throwable e) {
            System.out.println("stack length: " + oom.stackLength);
            e.printStackTrace();
        }
    }

    public void stackLeak() {
        stackLength++;
        stackLeak();
    }
}
