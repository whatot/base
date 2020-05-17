package com.mine.citest;

import java.util.*;

public class OnlyTry {

    public static void run() {
        Random rmd = new Random();
        ArrayList<Integer> keyList = new ArrayList<Integer>();
        for (int i = 0; i < 100000000; i++) {
            rmd.setSeed(i);
            int keyId = rmd.nextInt(1000000000);
            keyList.add(keyId);
        }
        Collections.sort(keyList);
        System.out.println("size: " + keyList.size());
        System.out.println("max: " + Collections.max(keyList));
        System.out.println("min: " + Collections.min(keyList));
    }

    public static void main(String[] args) {
        OnlyTry.run();
    }
}

