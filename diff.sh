#!/bin/bash
a=$1
b=$2
xxd $a > $a.hex
xxd $b > $b.hex
diff $a.hex $b.hex
rm *.hex
