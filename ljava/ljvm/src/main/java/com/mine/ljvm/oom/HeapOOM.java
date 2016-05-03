package com.mine.ljvm.oom;

/* VM options: -Xms20m -Xmx20m -XX:+HeapDumpOnOutOfMemoryError -XX:HeapDumpPath="./logs/"  */

import java.util.ArrayList;

public class HeapOOM {
    public static void main(String[] args) {
        ArrayList<OOMObject> list = new ArrayList<OOMObject>();
        while (true) {
            list.add(new OOMObject());
        }
    }

    static class OOMObject {

    }
}
