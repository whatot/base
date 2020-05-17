package com.mine.lasync;

import org.apache.http.*;
import org.apache.http.impl.nio.bootstrap.HttpServer;
import org.apache.http.impl.nio.bootstrap.ServerBootstrap;
import org.apache.http.impl.nio.reactor.IOReactorConfig;
import org.apache.http.nio.protocol.*;
import org.apache.http.protocol.HttpContext;

import java.io.IOException;
import java.util.concurrent.TimeUnit;

public class TaskServer {
    public static void main(String args[]) throws IOException {
        IOReactorConfig ioReactorConfig = IOReactorConfig.custom()
                .setSoTimeout(15000)
                .setTcpNoDelay(true)
                .build();

        final HttpServer httpServer = ServerBootstrap.bootstrap()
                .setServerInfo("simple http server/1.1")
                .setIOReactorConfig(ioReactorConfig)
                .setListenerPort(11111)
                .registerHandler("/basic", new BasicHandler())
                .create();
        httpServer.start();
        System.out.println("TaskServer is started");

        Runtime.getRuntime().addShutdownHook(new Thread() {
            @Override
            public void run() {
                httpServer.shutdown(5, TimeUnit.SECONDS);
            }
        });
    }

    private static class BasicHandler implements HttpAsyncRequestHandler {

        @Override
        public HttpAsyncRequestConsumer processRequest(HttpRequest request, HttpContext context)
                throws HttpException, IOException {
            return new BasicAsyncRequestConsumer();
        }

        @Override
        public void handle(final Object data, final HttpAsyncExchange httpExchange, final HttpContext context)
                throws HttpException, IOException {
            HttpResponse response = httpExchange.getResponse();
            handleInternal((HttpRequest) data, response, context);
            httpExchange.submitResponse(new BasicAsyncResponseProducer(response));
        }

        private void handleInternal(final HttpRequest request, final HttpResponse response, final HttpContext context) {
            AsyncUtils.sleep(20);
            // set default code
            response.setStatusCode(HttpStatus.SC_BAD_REQUEST);
            for (Header header : request.getAllHeaders()) {
                if (header.getName().equals("random")) {
                    int randomValue = Integer.parseInt(header.getValue());
                    if (randomValue % 3 == 0) {
                        response.setStatusCode(HttpStatus.SC_BAD_REQUEST);
                    } else {
                        response.setStatusCode(HttpStatus.SC_OK);
                    }
                }
            }
        }
    }
}
