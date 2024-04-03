#!/bin/sh

filename=stats.txt
command=./build/drogonBenchmark
port=8080

bench() {
    for i in $(seq 1 10); do
        {
            printf "Run %s: " "$i"
            wrk -t8 -c500 -d30s http://localhost:$port
            printf "\n"
        } >>$filename
    done
}

mainPid=$$
kill_handler() {
    kill $appPid
    kill $benchPid
    kill $killerPid
    kill $mainPid
}
trap kill_handler INT

$command &
appPid=$!

sleep 5

true >$filename

bench &
benchPid=$!

(
    sleep 320
    kill $appPid
) &
killerPid=$!

wait $appPid $benchPid
kill_handler
