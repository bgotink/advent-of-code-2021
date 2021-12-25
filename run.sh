#!/bin/bash

set -e

if ! command -v cargo >/dev/null 2>&1; then 
  echo "Couldn't find cargo" >&2
  exit 1
fi

cd "$(dirname "$0")"

input_filename="input.dat"

folder="$(pwd)"
bin=""
verbosity="--quiet"

while [ $# -gt 0 ]; do
  case "$1" in
    -e|--example)
      input_filename="test.dat"
      ;;
    -v|--verbose)
      verbosity=""
      ;;
    *)
      folder="$folder/$1"
      bin="$bin-$1"
      ;;
  esac
  shift
done

if [ -z $bin ]; then
  echo "Usage: run.sh [--example, -e] [--verbose, -v] <day> [part]" >&2
  echo "" >&2
  echo "Options:" >&2
  echo "  --example, -e         Use the example data given in the exercise description" >&2
  echo "  --verbose, -v         Print cargo compilation output" >&2
  echo "" >&2
  echo "Examples:" >&2
  echo "  run.sh 01 01" >&2
  echo "  run.sh 24" >&2
  exit 1
fi

if ! [ -f "$folder/$input_filename" ]; then
  echo "Can't find $input_filename for ${bin:1}" >&2
  exit 1
fi

cargo run $verbosity --bin "${bin:1}" "$folder/$input_filename"
