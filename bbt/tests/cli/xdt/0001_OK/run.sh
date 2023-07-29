#!/usr/bin/env bash

rm -rf output.html

dmntk xdt input.dtb output.html 2>&1

head -n 10 output.html

rm -rf output.html