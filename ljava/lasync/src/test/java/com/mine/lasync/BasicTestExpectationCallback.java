package com.mine.lasync;

import org.mockserver.mock.action.ExpectationCallback;
import org.mockserver.model.Header;
import org.mockserver.model.HttpRequest;
import org.mockserver.model.HttpResponse;

import java.util.concurrent.TimeUnit;

public class BasicTestExpectationCallback implements ExpectationCallback {
    private HttpResponse httpResponseOK = HttpResponse.response()
            .withStatusCode(200)
            .withHeaders(
                    new Header("Content-Type", "application/json; charset=utf-8"),
                    new Header("Cache-Control", "public, max-age=86400")
            )
            .withBody("{ message: 'incorrect username and password combination' }")
            .withDelay(TimeUnit.MILLISECONDS, 50);

    @Override
    public HttpResponse handle(HttpRequest httpRequest) {
        for (Header header : httpRequest.getHeaders()) {
            if (header.getName().getValue().equals("random")) {
                int value = Integer.parseInt(header.getValues().get(0).getValue());
                if (value % 3 == 0) {
                    return httpResponseOK;
                }
            }
        }
        return HttpResponse.notFoundResponse();
    }
}
