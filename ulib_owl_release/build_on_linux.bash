#!/bin/bash

WORK=$HOME/Develop/remote_builder
barch=$1

mkdir -p ${WORK} && echo "create dir: $WORK"
cd ${WORK} && echo "chage dir: $WORK"

git clone --branch $barch --single-branch https://github.com/Uliboooo/ghost_git_writer

cd ghost_git_writer
cargo build --release

mkdir -p ./release/

zip -j ./release/ggw_${2}_linux.zip ./target/release/ggw

cd .. && rm -r $WORK/ghost_git_writer
