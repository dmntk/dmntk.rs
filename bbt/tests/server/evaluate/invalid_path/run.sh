#!/usr/bin/env bash

dmntk srv -D . > /dev/null 2>&1 & 
_pid=$!
sleep 0.1

curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/io/dmntk/invalid/Greeting%20Message

kill -s SIGINT "$_pid"
sleep 0.1