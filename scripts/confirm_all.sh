#!/bin/bash
parallel -n 1 ./confirm.sh <$1 | tee o.confirmations.log
