##!/bin/sh
pmap $1 | tail -n1 | tr -cd ' 0123456789' | awk '{print $3}'
