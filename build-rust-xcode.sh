#!/bin/bash

export PATH="$PATH:${HOME}/.cargo/bin"

cd "${SRCROOT}/qlhighlight/"
if [[ ${ACTION:-build} = "build" ]]; then
    if [[ $CONFIGURATION = "Debug" ]]; then
        cargo build
    else
        cargo build --release
    fi
elif [[ $ACTION = "clean" ]]; then
        cargo clean
fi
