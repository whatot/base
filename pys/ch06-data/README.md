## ch06 data 数据

### OS模块

	In [1]: import os

	In [2]: os.
	os.EX_CANTCREAT      os.confstr_names     os.pardir
	os.EX_CONFIG         os.ctermid           os.path
	os.EX_DATAERR        os.curdir            os.pathconf
	os.EX_IOERR          os.defpath           os.pathconf_names
	os.EX_NOHOST         os.devnull           os.pathsep
	os.EX_NOINPUT        os.dup               os.pipe
	os.EX_NOPERM         os.dup2              os.popen
	os.EX_NOUSER         os.environ           os.popen2
	os.EX_OK             os.errno             os.popen3
	os.EX_OSERR          os.error             os.popen4
	os.EX_OSFILE         os.execl             os.putenv
	os.EX_PROTOCOL       os.execle            os.read
	os.EX_SOFTWARE       os.execlp            os.readlink
	os.EX_TEMPFAIL       os.execlpe           os.remove
	os.EX_UNAVAILABLE    os.execv             os.removedirs
	os.EX_USAGE          os.execve            os.rename
	os.F_OK              os.execvp            os.renames
	os.NGROUPS_MAX       os.execvpe           os.rmdir
	os.O_APPEND          os.extsep            os.sep
	os.O_ASYNC           os.fchdir            os.setegid
	os.O_CREAT           os.fchmod            os.seteuid
	os.O_DIRECT          os.fchown            os.setgid
	os.O_DIRECTORY       os.fdatasync         os.setgroups
	os.O_DSYNC           os.fdopen            os.setpgid
	os.O_EXCL            os.fork              os.setpgrp
	os.O_LARGEFILE       os.forkpty           os.setregid
	os.O_NDELAY          os.fpathconf         os.setresgid
	os.O_NOATIME         os.fstat             os.setresuid
	os.O_NOCTTY          os.fstatvfs          os.setreuid
	os.O_NOFOLLOW        os.fsync             os.setsid
	os.O_NONBLOCK        os.ftruncate         os.setuid
	os.O_RDONLY          os.getcwd            os.spawnl
	os.O_RDWR            os.getcwdu           os.spawnle
	os.O_RSYNC           os.getegid           os.spawnlp
	os.O_SYNC            os.getenv            os.spawnlpe
	os.O_TRUNC           os.geteuid           os.spawnv
	os.O_WRONLY          os.getgid            os.spawnve
	os.P_NOWAIT          os.getgroups         os.spawnvp
	os.P_NOWAITO         os.getloadavg        os.spawnvpe
	os.P_WAIT            os.getlogin          os.stat
	os.R_OK              os.getpgid           os.stat_float_times
	os.SEEK_CUR          os.getpgrp           os.stat_result
	os.SEEK_END          os.getpid            os.statvfs
	os.SEEK_SET          os.getppid           os.statvfs_result
	os.TMP_MAX           os.getresgid         os.strerror
	os.UserDict          os.getresuid         os.symlink
	os.WCONTINUED        os.getsid            os.sys
	os.WCOREDUMP         os.getuid            os.sysconf
	os.WEXITSTATUS       os.initgroups        os.sysconf_names
	os.WIFCONTINUED      os.isatty            os.system
	os.WIFEXITED         os.kill              os.tcgetpgrp
	os.WIFSIGNALED       os.killpg            os.tcsetpgrp
	os.WIFSTOPPED        os.lchown            os.tempnam
	os.WNOHANG           os.linesep           os.times
	os.WSTOPSIG          os.link              os.tmpfile
	os.WTERMSIG          os.listdir           os.tmpnam
	os.WUNTRACED         os.lseek             os.ttyname
	os.W_OK              os.lstat             os.umask
	os.X_OK              os.major             os.uname
	os.abort             os.makedev           os.unlink
	os.access            os.makedirs          os.unsetenv
	os.altsep            os.minor             os.urandom
	os.chdir             os.mkdir             os.utime
	os.chmod             os.mkfifo            os.wait
	os.chown             os.mknod             os.wait3
	os.chroot            os.name              os.wait4
	os.close             os.nice              os.waitpid
	os.closerange        os.open              os.walk
	os.confstr           os.openpty           os.write


### copy, mv, rename, delete data

### path, dir, file

### diff

### merge

### fnmatch

### rsync

### metadata

### tar

### tarfile模块 建立tar归档

### tarfile模块 检查tar文件内容
