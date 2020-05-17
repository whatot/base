package com.mine.citest;

import org.apache.cassandra.thrift.Cassandra;
import org.apache.cassandra.thrift.InvalidRequestException;
import org.apache.cassandra.thrift.UnavailableException;
import org.apache.log4j.Logger;
import org.apache.log4j.PropertyConfigurator;
import org.apache.thrift.TException;
import org.apache.thrift.protocol.TBinaryProtocol;
import org.apache.thrift.protocol.TProtocol;
import org.apache.thrift.transport.TFramedTransport;
import org.apache.thrift.transport.TSocket;
import org.apache.thrift.transport.TTransport;
import org.apache.thrift.transport.TTransportException;
import sun.misc.Signal;
import sun.misc.SignalHandler;

import java.net.InetAddress;
import java.net.UnknownHostException;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Random;
import java.util.concurrent.*;

import static java.util.Collections.shuffle;

public class CassTestService {
    private static final int maxThreadCount = 30;
    private static final int maxTaskLimit = 10000;
    private static boolean isNeedExit = false;
    private String hostAddress;
    private ThreadPoolExecutor executor;
    private Random rmd;
    private LinkedList<Integer> taskList;
    private LinkedList<Integer> returnErrorList;
    private Queue<Future> futureQueue;
    private Logger logger;

    public CassTestService() {
        this.logger = Logger.getLogger(CassTestService.class.getName() + " manager process");
        PropertyConfigurator.configure("log4j.properties");
        setHostAddressFromSystem();
        ThreadFactory cassThreadFactory = new CassThreadFactory();
        this.executor = (ThreadPoolExecutor) Executors.newCachedThreadPool(cassThreadFactory);
        this.executor.setCorePoolSize(maxThreadCount);
        this.rmd = new Random();
        this.futureQueue = new LinkedList<Future>();
        this.returnErrorList = new LinkedList<Integer>();
        truncateTestTable();
        initTaskList();
    }

    public static void main(String[] args) {
        CassTestService cass = new CassTestService();

        // signal process
        SignalHandler sh = new SignalHandler() {
            public void handle(Signal signal) {
                System.out.println("time to exit:" + System.currentTimeMillis());
                isNeedExit = true;
            }
        };

        try {
            Signal.handle(new Signal("HUP"), sh);
            Signal.handle(new Signal("INT"), sh);
            Signal.handle(new Signal("TERM"), sh);
        } catch (IllegalArgumentException e) {
            cass.logger.warn("can't register exit signal handler!");
            System.exit(-1);
        }

        for (; ; ) {
            if (isNeedExit) {
                cass.executor.shutdownNow();
                if (cass.executor.isTerminating()) {
                    cass.logger.warn("finishing all tasks!");
                }
                if (cass.returnErrorList.size() > 0) {
                    cass.logger.warn("error list is not empty, print all:");
                    while (cass.returnErrorList.size() > 0) {
                        cass.logger.warn("error in keyId: " + cass.returnErrorList.poll());
                    }
                }
                System.exit(0);
            } else {
                if (cass.taskList.isEmpty()) {
                    if (cass.futureQueue.isEmpty()) {
                        // when all tasks were done, all return were got, ready to next generation.
                        cass.logger.warn("reset all taskList, and truncate test table !!!");
                        cass.truncateTestTable();
                        cass.initTaskList();
                    } else {
                        // get remaining result
                        cass.checkThreadResult();
                    }
                } else {
                    int startPoint = cass.rmd.nextInt(cass.taskList.size());
                    cass.logger.info("start " + startPoint + " process");
                    Callable worker = new ContinueRunWorker(cass.taskList.get(startPoint), cass.hostAddress);
                    cass.taskList.remove(startPoint);
                    cass.futureQueue.add(cass.executor.submit(worker));

                    if (cass.futureQueue.size() > maxThreadCount * 5) {
                        cass.checkThreadResult();
                    }
                }
            }
        }
    }

    private void checkThreadResult() {
        Future task = this.futureQueue.poll();
        try {
            Object res = task.get();
            int keyId = (Integer) res;
            this.logger.info("task return keyId: " + keyId + ", " + this.taskList.size() + " tasks left!");
            if (keyId != 0) {
                this.returnErrorList.add(keyId);
                isNeedExit = true;
            }
        } catch (InterruptedException e) {
            this.logger.warn("Interrupted by System in task.get().");
        } catch (ExecutionException e) {
            this.logger.warn("Execution error in task.get().");
        }
    }

    private void initTaskList() {
        this.taskList = new LinkedList<Integer>();
        for (int i = 0; i < maxTaskLimit; i++) {
            taskList.add(i);
        }
        shuffle(taskList);
    }

    private void setHostAddressFromSystem() {
        try {
            this.hostAddress = InetAddress.getLocalHost().getHostAddress();
        } catch (UnknownHostException e) {
            this.logger.warn("Can not get hostAddress in setHostAddressFromSystem()");
            System.exit(-1);
        }
    }

    private void truncateTestTable() {
        TTransport tr = new TFramedTransport(new TSocket(this.hostAddress, 9160));
        TProtocol proto = new TBinaryProtocol(tr);
        Cassandra.Client client = new Cassandra.Client(proto);
        try {
            tr.open();
        } catch (TTransportException e) {
            this.logger.warn("Error in truncate table! " + "TTransportException");
        }

        try {
            client.set_keyspace("citest");
            client.truncate("ccitable");
        } catch (InvalidRequestException e) {
            this.logger.warn("Error in truncate table! " + "InvalidRequestException");
        } catch (UnavailableException e) {
            this.logger.warn("Error in truncate table! " + "UnavailableException ");
        } catch (TException e) {
            this.logger.warn("Error in truncate table! " + "TException ");
        }
        this.logger.info("Success in truncate test table");
    }
}
