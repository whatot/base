# netease music container

Because netease-music always crash in arch,
so I create a container based on ubuntu16.04 to run it.

* https://blog.jessfraz.com/post/docker-containers-on-the-desktop/
* https://github.com/jessfraz/dockerfiles


## problem(unsolved)

https://github.com/jessfraz/dockerfiles/issues/156

because archlinux default kernel don't support user namespace by default,
so the error below will happen, even run after `xhost local:root`.
```
Failed to move to new namespace: PID namespaces supported, Network namespace supported, but failed: errno = Operation not permitted
```

before `xhost local:root`, the error below will happen also.
```
No protocol specified
QXcbConnection: Could not connect to display :1
/usr/bin/netease-cloud-music: line 8:     7 Aborted                 (core dumped) /usr/lib/netease-cloud-music/netease-cloud-music
```
