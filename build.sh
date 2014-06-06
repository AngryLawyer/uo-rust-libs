#!/bin/bash

function compile {
    rustc src/lib.rs --out-dir bin
}

function test {
    rustc src/UORustLibs.rc -o bin/test --test && bin/test
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
