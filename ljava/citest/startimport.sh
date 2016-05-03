#!/usr/bin/env bash

java -Xmx100M -Xms40M -XX:+HeapDumpOnOutOfMemoryError -XX:+UseParNewGC -XX:+UseConcMarkSweepGC  -cp ~/citest-1.0-SNAPSHOT-jar-with-dependencies.jar com.mine.citest.CassTestService
