# lgo

only to archive the code snippets, or test

## a useful Makefile for go projects

https://github.com/cloudflare/hellogopher/

init setup
```
wget https://raw.githubusercontent.com/cloudflare/hellogopher/master/Makefile
$EDITOR Makefile # modify IMPORT_PATH
make setup
git add Makefile .gitignore vendor/
```

add more depends into vendor
```
./bin/gvt fetch github.com/fatih/color
git add vendor/
```
