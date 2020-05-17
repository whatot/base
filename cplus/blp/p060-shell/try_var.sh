#!/bin/sh

salutation="Hello"
echo $salutation
echo "The program \$0 $0 is now running"
echo "The second parameter was \$2 $2"
echo "The first parameter was \$1 $1"
echo "The parameter list was \$* $*"
echo "The user's home directory is \$HOME $HOME"

echo "Please enter a new greeting"
read salutation

echo $salutation
echo "The script is now complete"
echo 0

