#!/usr/bin/env bash

dmntk srv -c never 2>&1 & 
_pid_a=$!
sleep 0.1

dmntk srv -c never 2>&1

kill -s SIGINT "$_pid_a"
sleep 0.1