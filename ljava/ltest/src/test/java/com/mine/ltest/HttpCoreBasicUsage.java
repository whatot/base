package com.mine.ltest;

import org.apache.http.*;
import org.apache.http.config.SocketConfig;
import org.apache.http.message.BasicHttpRequest;
import org.apache.http.message.BasicHttpResponse;
import org.junit.Assert;
import org.junit.Test;

public class HttpCoreBasicUsage {
    @Test
    public void HttpRequestMessage() {
        HttpRequest request = new BasicHttpRequest("GET", "/", HttpVersion.HTTP_1_1);
        // System.out.println(request.getRequestLine().toString());
        Assert.assertTrue(request.getRequestLine().toString().equals("GET / HTTP/1.1"));
    }

    @Test
    public void HttpResponseMessage() {
        HttpResponse response = new BasicHttpResponse(HttpVersion.HTTP_1_1, HttpStatus.SC_OK, "OK");
        response.addHeader("Set-Cookie", "c1=a; path=/; domain=localhost");
        response.addHeader("Set-Cookie", "c2=b; path=\"/\", c3=c; domain=\"localhost\"");
        response.addHeader("Result", "success");

        // System.out.println(response.getStatusLine().toString());
        Assert.assertTrue(response.getStatusLine().toString().equals("HTTP/1.1 200 OK"));
        Assert.assertEquals(response.getStatusLine().getProtocolVersion(), HttpVersion.HTTP_1_1);
        Assert.assertEquals(response.getStatusLine().getStatusCode(), HttpStatus.SC_OK);

        HeaderIterator it = response.headerIterator();
        while (it.hasNext()) {
            Header item = it.nextHeader();
            // System.out.println(item.getName() + " " + item.getValue());
            if (item.getName().equals("Result")) {
                Assert.assertTrue(item.getValue().equals("success"));
            }
            if (item.getName().equals("Set-Cookie")) {
                Assert.assertTrue(item.getValue().length() > 10);
            }
        }
    }

    @Test
    public void SocketConfig() {
        SocketConfig socketConfig = SocketConfig.DEFAULT;
        // System.out.println("SocketConfig: " + socketConfig.toString());
        Assert.assertEquals(socketConfig.getSoTimeout(), 0);
        Assert.assertEquals(socketConfig.isSoReuseAddress(), false);
        Assert.assertEquals(socketConfig.getSoLinger(), -1);
        Assert.assertEquals(socketConfig.isSoKeepAlive(), false);
        Assert.assertEquals(socketConfig.isTcpNoDelay(), true);
        Assert.assertEquals(socketConfig.getSndBufSize(), 0);
        Assert.assertEquals(socketConfig.getRcvBufSize(), 0);
        Assert.assertEquals(socketConfig.getBacklogSize(), 0);
    }
}
