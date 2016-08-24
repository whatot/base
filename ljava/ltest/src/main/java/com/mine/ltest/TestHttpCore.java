package com.mine.ltest;

import org.apache.http.*;
import org.apache.http.config.SocketConfig;
import org.apache.http.message.BasicHttpRequest;
import org.apache.http.message.BasicHttpResponse;

import java.util.logging.Level;
import java.util.logging.Logger;

public class TestHttpCore {
    private final Logger logger;

    public TestHttpCore() {
        logger = Logger.getLogger(TestHttpCore.class.getName());
        logger.setLevel(Level.INFO);
    }

    public static void main(String[] args) {
        TestHttpCore testcase = new TestHttpCore();
        testcase.testHttpRequestMessage();
        testcase.testHttpResponseMessage();
        testcase.testSocketConfig();
    }

    private void testHttpRequestMessage() {
        HttpRequest request = new BasicHttpRequest("GET", "/", HttpVersion.HTTP_1_1);

        logger.info(request.getRequestLine().toString());
    }

    private void testHttpResponseMessage() {
        HttpResponse response = new BasicHttpResponse(HttpVersion.HTTP_1_1, HttpStatus.SC_OK, "OK");
        response.addHeader("Set-Cookie", "c1=a; path=/; domain=localhost");
        response.addHeader("Set-Cookie", "c2=b; path=\"/\", c3=c; domain=\"localhost\"");
        response.addHeader("Result", "success");

        logger.info(response.getStatusLine().toString());
        HeaderIterator it = response.headerIterator();
        while (it.hasNext()) {
            logger.info(it.next().toString());
        }
    }

    private void testSocketConfig() {
        SocketConfig socketConfig = SocketConfig.DEFAULT;

        logger.info("SocketConfig: " + socketConfig.toString());
    }
}
