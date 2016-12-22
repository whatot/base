package com.mine.lasync;

import org.apache.http.*;
import org.apache.http.concurrent.FutureCallback;
import org.apache.http.message.BasicHttpRequest;
import org.apache.http.util.EntityUtils;

import java.util.Random;
import java.util.concurrent.atomic.AtomicInteger;

public class TaskClient {
    private static final AtomicInteger completed = new AtomicInteger();
    private static final AtomicInteger completedOk = new AtomicInteger();
    private static final AtomicInteger failed = new AtomicInteger();
    private static final AtomicInteger cancelled = new AtomicInteger();
    private static HttpHost httpHost = new HttpHost("127.0.0.1", 11111);
    private static Random random = new Random();

    public static void main(String args[]) {
        NSAsyncSingleton.instance().start();
        for (int i = 0; i < 5000; i++) {
            putSingleTask();
        }
        System.out.println("all tasks have been added");

        for (int i = 0; i < 30; i++) {
            AsyncUtils.sleep(1000);
            printStat();
        }

        Runtime.getRuntime().addShutdownHook(new Thread() {
            @Override
            public void run() {
                NSAsyncSingleton.instance().close();
            }
        });
    }

    private static void printStat() {
        System.out.print("completed: " + completed.get() + "; completedOk: " + completedOk.get());
        System.out.print("; failed: " + failed.get() + "; cancelled: " + cancelled.get());
        System.out.println();
    }

    private static void putSingleTask() {
        final HttpRequest request = new BasicHttpRequest("GET", "/");
        request.setHeader("random", String.valueOf(random.nextInt()));
        NSAsyncSingleton.instance().execute(httpHost, request, new FutureCallback<HttpResponse>() {
            @Override
            public void completed(HttpResponse result) {
                completed.incrementAndGet();
                if (result.getStatusLine().getStatusCode() == HttpStatus.SC_OK) {
                    completedOk.incrementAndGet();
                }
                HttpEntity entity = result.getEntity();
                if (entity != null) {
                    EntityUtils.consumeQuietly(entity);
                }
            }

            @Override
            public void failed(Exception ex) {
                failed.incrementAndGet();
            }

            @Override
            public void cancelled() {
                cancelled.incrementAndGet();
            }
        });
    }
}
