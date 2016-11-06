# golang about

move from changing

## useful articles or sites

* https://golang.org/doc/
* https://play.golang.org/

### todo

* https://tour.golang.org/welcome/1
* https://gobyexample.com/

* http://golang.org/pkg/encoding/json/
* http://blog.golang.org/2011/01/json-and-go.html
* http://blog.golang.org/2011/07/error-handling-and-go.html
* http://jordanorelli.tumblr.com/post/32665860244/how-to-use-interfaces-in-go
* http://talks.godoc.org/github.com/davecheney/high-performance-go-workshop/high-performance-go-workshop.slide#1
* https://github.com/davecheney/high-performance-go-workshop
* https://www.zhihu.com/question/23486344

### done

* https://github.com/golang/go/wiki/PackageManagementTools
* https://blog.golang.org/errors-are-values
* https://golang.org/doc/effective_go.html
* https://talks.golang.org/2012/splash.article  Go at Google
* https://golang.org/doc/code.html
* https://golang.org/doc/install
* http://devs.cloudimmunity.com/gotchas-and-common-mistakes-in-go-golang/

## glide

```
$ go get -u -v github.com/Masterminds/glide

$ glide create                            # Start a new workspace
$ vim glide.yaml                          # and edit away!
$ glide get github.com/Masterminds/cookoo # Get a package and add to glide.yaml
$ glide install                           # Install packages and dependencies
>>> work, work, work >>>
$ go build                                # Go tools work normally
$ glide up                                # Update to newest versions of the package
```

## govender

```
go get -u -v github.com/kardianos/govendor
govendor init
govendor add +external
govendor list
```

