#!/bin/bash
solver-cli -s r_$1.sh $1
if [ 0 == $? ]; then
    chmod a+x r_$1.sh
    creduce --not-c --timeout 12 ./r_$1.sh $1
    rm r_$1.sh
else
    mkdir -p no-repro
    mv $1 no-repro
fi
