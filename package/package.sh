#!/bin/bash

rm -rf ./target
rm -f "branch-dashboard-$1.zip"

mkdir ./target
cp ./server/target/release/branch-dashboard target/
cp -r ./server/static target/