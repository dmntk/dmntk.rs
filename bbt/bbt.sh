#!/usr/bin/env bash

CURRENT_DIR=$(pwd)
PARENT_DIR=$(realpath "$CURRENT_DIR/..")
DMNTK_PATH=$PARENT_DIR/target/debug
DMNTK_BINARY=$DMNTK_PATH/dmntk

if [ ! -f "$DMNTK_BINARY" ] ; then
  echo "DMNTK binary not found in path: $DMNTK_PATH"
  exit 1
fi

PATH=$DMNTK_PATH:$PATH ./run.sh "$@"