package com.mine.lasync;

import org.apache.http.HttpHost;
import org.junit.After;
import org.junit.Before;
import org.junit.Test;
import org.mockserver.integration.ClientAndProxy;
import org.mockserver.integration.ClientAndServer;
import org.mockserver.matchers.Times;
import org.mockserver.model.HttpCallback;
import org.mockserver.model.HttpRequest;

import static org.mockserver.integration.ClientAndProxy.startClientAndProxy;
import static org.mockserver.integration.ClientAndServer.startClientAndServer;

public class TaskClientTest {
    private ClientAndProxy proxy;
    private ClientAndServer mockServer;
    private HttpHost httpHost;

    @Before
    public void setUp() throws Exception {
        mockServer = startClientAndServer(10111);
        proxy = startClientAndProxy(10112);
        httpHost = new HttpHost("127.0.0.1", mockServer.getPort());
        mockServer
                .when(HttpRequest.request()
                        .withMethod("GET")
                        .withPath("/basic")
                        .withHeader("random"), Times.unlimited())
                .callback(HttpCallback.callback()
                        .withCallbackClass(BasicTestExpectationCallback.class.getName())
                );
    }

    @After
    public void tearDown() throws Exception {
        proxy.stop();
        mockServer.stop();
    }

    @Test
    public void testStartTask() throws Exception {
        TaskClient.startTask(httpHost, 50);
    }

}