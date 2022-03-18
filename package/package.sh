#!/bin/bash

rm -rf ./target
rm -f "gitlab-branch-dashboard-$1.zip"

mkdir ./target
cp ./server/target/release/gitlab-branch-dashboard target/
cp -r ./server/static target/