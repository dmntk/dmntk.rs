#!/usr/bin/env bash

dmntk tdt -c never input.test input.dtb 2>&1

dmntk tdt input.test input.dtb 2>&1 | sed -r 's/\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[m|K]//g'