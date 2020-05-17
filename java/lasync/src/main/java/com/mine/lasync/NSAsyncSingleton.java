package com.mine.lasync;

import org.apache.http.HttpHost;
import org.apache.http.HttpRequest;
import org.apache.http.HttpResponse;
import org.apache.http.concurrent.FutureCallback;
import org.apache.http.config.ConnectionConfig;
import org.apache.http.config.Registry;
import org.apache.http.config.RegistryBuilder;
import org.apache.http.impl.nio.client.CloseableHttpAsyncClient;
import org.apache.http.impl.nio.client.HttpAsyncClients;
import org.apache.http.impl.nio.conn.PoolingNHttpClientConnectionManager;
import org.apache.http.impl.nio.reactor.DefaultConnectingIOReactor;
import org.apache.http.impl.nio.reactor.IOReactorConfig;
import org.apache.http.nio.conn.NHttpClientConnectionManager;
import org.apache.http.nio.conn.NoopIOSessionStrategy;
import org.apache.http.nio.conn.SchemeIOSessionStrategy;
import org.apache.http.nio.conn.ssl.SSLIOSessionStrategy;
import org.apache.http.nio.reactor.ConnectingIOReactor;
import org.apache.http.nio.reactor.IOReactorException;
import org.apache.http.ssl.SSLContexts;

import javax.net.ssl.SSLContext;
import java.io.Closeable;
import java.nio.charset.CodingErrorAction;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;

public class NSAsyncSingleton implements Closeable {
    private CloseableHttpAsyncClient httpAsyncClient;
    private IdleConnectionMonitorThread staleMonitor;

    private NSAsyncSingleton() throws IOReactorException {
        IOReactorConfig ioReactorConfig = IOReactorConfig.custom()
                .setIoThreadCount(Runtime.getRuntime().availableProcessors())
                .setConnectTimeout(10000)
                .setSoTimeout(20000)
                .setSoKeepAlive(true)
                .build();
        ConnectingIOReactor ioReactor = new DefaultConnectingIOReactor(ioReactorConfig);

        SSLContext sslcontext = SSLContexts.createSystemDefault();
        Registry<SchemeIOSessionStrategy> sessionStrategyRegistry =
                RegistryBuilder.<SchemeIOSessionStrategy>create()
                        .register("http", NoopIOSessionStrategy.INSTANCE)
                        .register("https", new SSLIOSessionStrategy(sslcontext))
                        .build();

        PoolingNHttpClientConnectionManager cm = new
                PoolingNHttpClientConnectionManager(ioReactor, sessionStrategyRegistry);
        cm.setMaxTotal(20);
        cm.setDefaultMaxPerRoute(20);

        ConnectionConfig connectionConfig = ConnectionConfig.custom()
                .setMalformedInputAction(CodingErrorAction.IGNORE)
                .setUnmappableInputAction(CodingErrorAction.IGNORE)
                .setCharset(StandardCharsets.UTF_8)
                .build();
        cm.setDefaultConnectionConfig(connectionConfig);

        // pipeline is enabled by default
        httpAsyncClient = HttpAsyncClients.custom()
                .setConnectionManager(cm)
                .setConnectionManagerShared(false)
                .build();

        staleMonitor = new IdleConnectionMonitorThread(cm);
    }

    public static NSAsyncSingleton instance() {
        return Holder.INSTANCE;
    }

    public void start() {
        httpAsyncClient.start();
        staleMonitor.start();

        Runtime.getRuntime().addShutdownHook(new Thread() {
            @Override
            public void run() {
                NSAsyncSingleton.instance().close();
            }
        });
    }

    public void close() {
        if (staleMonitor.isAlive()) {
            staleMonitor.shutdown();
        }
        try {
            if (httpAsyncClient != null && httpAsyncClient.isRunning()) {
                httpAsyncClient.close();
            }
        } catch (Exception e) {
            // ignore it
        }
    }

    // copy more execute if need
    public Future<HttpResponse> execute(final HttpHost target,
                                        final HttpRequest request,
                                        final FutureCallback<HttpResponse> callback) {
        return httpAsyncClient.execute(target, request, callback);
    }

    private static class Holder {
        private static final NSAsyncSingleton INSTANCE;

        static {
            try {
                INSTANCE = new NSAsyncSingleton();
            } catch (IOReactorException e) {
                throw new ExceptionInInitializerError(e);
            }
        }
    }

    private static class IdleConnectionMonitorThread extends Thread {
        private final NHttpClientConnectionManager connMgr;
        private volatile boolean shutdown;

        IdleConnectionMonitorThread(NHttpClientConnectionManager connMgr) {
            super();
            this.connMgr = connMgr;
        }

        @Override
        public void run() {
            try {
                while (!shutdown) {
                    synchronized (this) {
                        wait(5000);
                        connMgr.closeExpiredConnections();
                        connMgr.closeIdleConnections(30, TimeUnit.SECONDS);
                    }
                }
            } catch (InterruptedException ex) {
                shutdown();
            }
        }

        void shutdown() {
            shutdown = true;
            synchronized (this) {
                notifyAll();
            }
        }
    }
}
