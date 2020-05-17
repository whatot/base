package com.mine.citest;


import org.apache.cassandra.thrift.*;
import org.apache.log4j.Logger;
import org.apache.log4j.PropertyConfigurator;
import org.apache.thrift.protocol.TBinaryProtocol;
import org.apache.thrift.protocol.TProtocol;
import org.apache.thrift.transport.TFramedTransport;
import org.apache.thrift.transport.TSocket;
import org.apache.thrift.transport.TTransport;

import java.io.UnsupportedEncodingException;
import java.nio.ByteBuffer;
import java.util.Random;
import java.util.Stack;
import java.util.concurrent.Callable;
import java.util.concurrent.TimeUnit;

public class ContinueRunWorker implements Callable {
    private static final ConsistencyLevel insertConsistencyLevel = ConsistencyLevel.QUORUM;
    private static final ConsistencyLevel getConsistencyLevel = ConsistencyLevel.QUORUM;
    private static final int maxKeyIdLimit = 1000000000;
    private static final int singleInsertLimit = 100;
    private int startPoint;
    private String hostAddress;
    private Logger logger;

    public ContinueRunWorker(int startPoint, String hostAddress) {
        this.startPoint = startPoint;
        this.hostAddress = hostAddress;
        this.logger = Logger.getLogger(ContinueRunWorker.class.getName());
        PropertyConfigurator.configure("log4j.properties");
    }

    public static ByteBuffer toByteBuffer(String value)
            throws UnsupportedEncodingException {
        return ByteBuffer.wrap(value.getBytes("UTF-8"));
    }

    public static String toString(ByteBuffer buffer)
            throws UnsupportedEncodingException {
        byte[] bytes = new byte[buffer.remaining()];
        buffer.get(bytes);
        return new String(bytes, "UTF-8");
    }

    public static String bytesToHex(ByteBuffer bytes) {
        final int offset = bytes.position();
        final int size = bytes.remaining();
        final char[] c = new char[size * 2];
        for (int i = 0; i < size; i++) {
            final int bint = bytes.get(i + offset);
            c[i * 2] = Hex.byteToChar[(bint & 0xf0) >> 4];
            c[1 + i * 2] = Hex.byteToChar[bint & 0x0f];
        }
        return Hex.wrapCharArray(c);
    }

    private void compareResKeyHashCode(int keyId, String resValue, String keyHashCode) {
        if (!resValue.equals(keyHashCode)) {
            this.logger.warn("read error! keyId:" + keyId + "; hashCode:" + keyHashCode + "; value:" + resValue);
        }
    }

    private void sleep(int second) {
        try {
            TimeUnit.SECONDS.sleep(second);
        } catch (InterruptedException e) {
            logger.warn("sleep interrupted by system.");
        }
    }

    public Object call() {
        try {
            Random allRandom = new Random();
            Stack<Integer> readStack = new Stack<Integer>();

            // connect to cassandra
            TTransport tr = new TFramedTransport(new TSocket(this.hostAddress, 9160));
            TProtocol proto = new TBinaryProtocol(tr);
            Cassandra.Client client = new Cassandra.Client(proto);
            tr.open();
            client.set_keyspace("citest");

            // insert process
            ColumnParent parent = new ColumnParent("ccitable");  // for insert
            Column valueColumn = new Column(toByteBuffer("value"));  // value column name
            this.logger.debug("insert " + this.startPoint + " process");
            for (int i = 0; i < singleInsertLimit; i++) {
                // generate random data
                allRandom.setSeed(this.startPoint * singleInsertLimit + i);
                int keyId = allRandom.nextInt(maxKeyIdLimit);
                readStack.push(keyId);
                ByteBuffer keyIdByteBuffer = toByteBuffer(String.valueOf(keyId));

                // insert data
                String keyHashCode = String.valueOf(String.valueOf(keyId).hashCode());
                ByteBuffer valueByteBuffer = toByteBuffer(keyHashCode);
                valueColumn.setValue(valueByteBuffer);
                long timestamp = System.currentTimeMillis();
                valueColumn.setTimestamp(timestamp);
                client.insert(keyIdByteBuffer, parent, valueColumn, insertConsistencyLevel);
            }

            // read and check process
            ColumnPath path = new ColumnPath("ccitable");  // for read
            path.setColumn(toByteBuffer("value"));
            this.logger.debug("read and check " + this.startPoint + " process");
            while (!readStack.isEmpty()) {
                int keyId = readStack.pop();
                ByteBuffer keyIdByteBuffer = toByteBuffer(String.valueOf(keyId));
                String keyHashCode = String.valueOf(String.valueOf(keyId).hashCode());

                try {
                    ColumnOrSuperColumn res = client.get(keyIdByteBuffer, path, getConsistencyLevel);
                    String resValue = toString(res.column.value);
                    compareResKeyHashCode(keyId, resValue, keyHashCode);
                } catch (NotFoundException e) {
                    this.logger.warn("Important!! Not Found keyId:" + keyId + " Try again!");
                    this.logger.warn("byte2hex: " + ContinueRunWorker.bytesToHex(keyIdByteBuffer));
                    e.printStackTrace();
                    System.exit(-1);

                    // delay 10s for reading
                    this.sleep(10);

                    // try twice
                    try {
                        ColumnOrSuperColumn res = client.get(keyIdByteBuffer, path, getConsistencyLevel);
                        String resValue = toString(res.column.value);
                        compareResKeyHashCode(keyId, resValue, keyHashCode);
                    } catch (NotFoundException e2) {
                        this.logger.warn("Important!! Not Found keyId:" + keyId + " Return the Key!");
                        return keyId;
                    }
                }
            }

            tr.close();

        } catch (Exception e) {
            this.logger.warn("Even exception, it goes on.");
            e.printStackTrace();
            System.exit(-1);
        }
        // keyId can not == 0
        return 0;
    }
}
