#!/bin/bash
xxd hello.class > hello.hex
xxd hello2.class > hello2.hex
diff hello.hex hello2.hex
rm *.hex
