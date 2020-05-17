package com.mine.citest;

import java.util.concurrent.ThreadFactory;

public class CassThreadFactory implements ThreadFactory {
    public Thread newThread(Runnable r) {
        Thread t = new Thread(r, "Task Thread");
        t.setDaemon(true);
        t.setPriority(Thread.NORM_PRIORITY);
        return t;
    }
}
