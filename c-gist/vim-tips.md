# vim tips

## vim: remove the first character of each line and append
(http://stackoverflow.com/questions/16139460/vim-remove-the-first-character-of-each-line-and-append)

```
:%s/^\([1-3]\),\(.*\)/\2,\1/g
```
