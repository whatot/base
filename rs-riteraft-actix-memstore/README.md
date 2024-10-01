# riteraft-warp-memstore

- https://github.com/PsiACE/riteraft 
  - RiteRaft - A raft framework, for regular people
- https://github.com/PsiACE/riteraft/tree/main/examples/riteraft-warp-memstore
- https://github.com/PsiACE/riteraft/tree/main/examples/riteraft-memstore

## test cmds

```shell

target/debug/rs-riteraft-warp-memstore --raft-addr 127.0.0.1:3001 --web-server 127.0.0.1:3002
target/debug/rs-riteraft-warp-memstore --raft-addr 127.0.0.1:4001 --peer-addr 127.0.0.1:3001 --web-server 127.0.0.1:4002
target/debug/rs-riteraft-warp-memstore --raft-addr 127.0.0.1:5001 --peer-addr 127.0.0.1:3001 --web-server 127.0.0.1:5002
target/debug/rs-riteraft-warp-memstore --raft-addr 127.0.0.1:6001 --peer-addr 127.0.0.1:3001
target/debug/rs-riteraft-warp-memstore --raft-addr 127.0.0.1:7001 --peer-addr 127.0.0.1:3001

curl 127.0.0.1:3002/put/v1/v1sss
curl 127.0.0.1:3002/get/v1

```