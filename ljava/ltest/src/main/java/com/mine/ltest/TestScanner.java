package com.mine.ltest;


import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.UUID;

public class TestScanner {

    private static final int PROCESS_TASK_LIMIT = 1000;

    public static void main(String[] args) {
        System.out.println("Working Directory = " + System.getProperty("user.dir"));
        final String testFile = "test.data";
        TestScanner.generateTestFile(testFile);
        TestScanner.processAllTasks(testFile);
    }

    private static void processAllTasks(String processFilePath) {
        String line;
        List<String> keyList = new ArrayList<String>(PROCESS_TASK_LIMIT);

        try {
            Scanner sc = new Scanner(new File(processFilePath));
            while (sc.hasNextLine()) {
                line = sc.nextLine().trim();
                if (line.length() > 0) {
                    keyList.add(line);
                }
                if (keyList.size() == PROCESS_TASK_LIMIT) {
                    processTaskList(keyList);
                }
            }
            if (!keyList.isEmpty()) {
                processTaskList(keyList);
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private static void processTaskList(List<String> taskList) {
        System.out.println("The processed taskList size:" + taskList.size());
        taskList.clear();
    }

    private static void generateTestFile(String filePath) {
        final int Min = 2000;
        final int Max = 20000;
        final int cycle = Min + (int) (Math.random() * ((Max - Min) + 1));
        System.out.println("The line of Test file:" + cycle);
        BufferedWriter writer;
        try {
            writer = new BufferedWriter(new FileWriter(filePath));
            for (int i = 0; i < cycle; i++) {
                writer.write(UUID.randomUUID().toString());
                writer.write("\n");
            }
            writer.flush();
            writer.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
