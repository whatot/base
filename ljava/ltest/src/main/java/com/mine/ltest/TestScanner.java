package com.mine.ltest;


import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.UUID;
import java.util.logging.Level;
import java.util.logging.Logger;

public class TestScanner {

    private static final int PROCESS_TASK_LIMIT = 1000;
    private final Logger logger;

    public TestScanner() {
        logger = Logger.getLogger(TestScanner.class.getName());
        logger.setLevel(Level.INFO);
        logger.info("Working Directory = " + System.getProperty("user.dir"));
    }

    public static void main(String[] args) {
        final String testFile = "test.data";
        TestScanner testcase = new TestScanner();
        testcase.generateTestFile(testFile);
        testcase.processAllTasks(testFile);
    }

    private void processAllTasks(String processFilePath) {
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

    private void processTaskList(List<String> taskList) {
        logger.info("The processed taskList size:" + taskList.size());
        taskList.clear();
    }

    private void generateTestFile(String filePath) {
        final int Min = 2000;
        final int Max = 20000;
        final int cycle = Min + (int) (Math.random() * ((Max - Min) + 1));
        logger.info("The line of Test file:" + cycle);
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
