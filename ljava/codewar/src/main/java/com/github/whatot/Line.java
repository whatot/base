package com.github.whatot;


public class Line {
    public static String Tickets(int[] peopleInLine)
    {
        if (peopleInLine == null || peopleInLine.length == 0) {
            return "YES";
        }

        int fifty = 0;
        int twentyFive = 0;

        for (int i : peopleInLine) {
            if (i == 100) {
                if (fifty > 0) {
                    fifty--;
                    twentyFive--;
                } else {
                    twentyFive-=3;
                }
            } else if (i == 50) {
                fifty++;
                twentyFive--;
            } else if (i == 25) {
                twentyFive++;
            } else {
                return "NO";
            }

            if (fifty < 0 || twentyFive < 0) {
                return "NO";
            }
        }

        return "YES";
    }

}
