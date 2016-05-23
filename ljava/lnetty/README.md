# learn netty


# run examples

## echo server

* server ``java -cp ./target/lnetty-1.0-SNAPSHOT-jar-with-dependencies.jar com.mine.lnetty.echoserver.EchoServer 11111``
* client ``java -cp ./target/lnetty-1.0-SNAPSHOT-jar-with-dependencies.jar com.mine.lnetty.echoserver.EchoClient 127.0.0.1 11111``

## time server

* server ``java -cp ./target/lnetty-1.0-SNAPSHOT-jar-with-dependencies.jar com.mine.lnetty.timeserver.TimeServer 127.0.0.1 8080``
* client ``java -cp ./target/lnetty-1.0-SNAPSHOT-jar-with-dependencies.jar com.mine.lnetty.timeserver.TimeClient 127.0.0.1 8080``
