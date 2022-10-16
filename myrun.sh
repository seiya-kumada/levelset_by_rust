#!/bin/bash

./target/debug/levelset_by_rust \
    --dim 2 \
    --verbose \
    --input-path ./hoge/foo.stl \
    --wband 5 \
    --wreset 2 \
    --time-step 1 \
    --gain 0.3 \
    --constant-speed 1 \
    --speed-threshold 0.05 \
    --left 484 \
    --top 153 \
    --right 502 \
    --bottom 181
