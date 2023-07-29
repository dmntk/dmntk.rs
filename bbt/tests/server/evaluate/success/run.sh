#!/usr/bin/env bash

dmntk srv -D . > /dev/null 2>&1 & 
_pid=$!
sleep 0.1

curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/io/dmntk/2_0001/Greeting%20Message
curl -s -d '{"Monthly Salary":12000}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/io/dmntk/2_0002/Yearly%20Salary

kill -s SIGINT "$_pid"
sleep 0.1