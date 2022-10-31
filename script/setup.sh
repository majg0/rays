#!/bin/bash

p=$(git rev-parse --show-toplevel)

cp $p/script/hooks/* "$p/.git/hooks/"

