package com.mine.ljvm.oom;

/* VM options: -Xss512k
 this class will run until it make full use of the system memory,
  so it is better run it in a VM */

public class JavaVMStackOOM {
    private int threadNum = 0;

    public static void main(String[] args) {
        JavaVMStackOOM oom = new JavaVMStackOOM();
        oom.stackLeakByThread();
    }

    private void dontStop() {
        while (true) {
        }
    }

    public void stackLeakByThread() {
        while (true) {
            Thread thread = new Thread(new Runnable() {
                public void run() {
                    dontStop();
                }
            });
            thread.start();
            System.out.println(this.threadNum++);
        }
    }
}
