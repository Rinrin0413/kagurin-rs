##!/bin/sh
ps -o etime= -p $1 | tr -d ' '
