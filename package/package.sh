#!/bin/bash

rm -rf ./target
rm -f "git-server-dashboard-$1.zip"

mkdir ./target
cp ./server/target/release/git-server-dashboard target/
cp -r ./server/static target/

cd target && zip -r "../git-server-dashboard-$1.zip" ** && cd -