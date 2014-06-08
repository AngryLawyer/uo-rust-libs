#!/bin/bash

function compile {
    mkdir bin
    rustc src/lib.rs --out-dir bin
}

function test {
    mkdir bin
    rustc --test src/lib.rs -o bin/test && bin/test
    rm bin/*.mul
    rm bin/*.idx
}

if [ "$#" -eq 0 ]; then
    compile
else
    while getopts ":ct" optname
            do
    case "$optname" in
            "c")
                compile
                ;;
            "t")
                test
                ;;
            "?")
                echo "Unknown option $OPTARG"
                ;;
            *)
                # Should not occur
                echo "I have no idea what just happened"
                ;;
        esac
    done
fi
