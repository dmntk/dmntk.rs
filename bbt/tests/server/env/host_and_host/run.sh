#!/usr/bin/env bash

DMNTK_HOST=127.0.0.1 dmntk srv -c never --host=localhost 2>&1 &
_pid=$!
sleep 0.1

kill -s SIGINT "$_pid"
sleep 0.1