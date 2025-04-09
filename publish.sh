#!/usr/bin/env bash

PUBLISHED_VERSION="0.3.7"

publish() {
  echo "======================================================================"
  echo " Publishing $1 ($3/17)                                                "
  echo "======================================================================"
  set -x
  cargo update
  cd "$1" && cargo publish && cd ..
  sed -i -E "s/$2 = .+/$2 = \"$PUBLISHED_VERSION\"/g" Cargo.toml
  set +x
  echo "======================================================================"
  echo " Published $1 ($3/17)                                                 "
  echo "======================================================================"
  echo "...sleeping 5s..."
  sleep 10s
}

publish_all() {
  publish "examples"          "dmntk-examples"             1
  publish "macros"            "dmntk-macros"               2
  publish "common"            "dmntk-common"               3
  publish "feel-number"       "dmntk-feel-number"          4
  publish "feel-temporal"     "dmntk-feel-temporal"        5
  publish "feel-grammar"      "dmntk-feel-grammar"         6
  publish "feel"              "dmntk-feel"                 7
  publish "feel-parser"       "dmntk-feel-parser"          8
  publish "feel-evaluator"    "dmntk-feel-evaluator"       9
  publish "model"             "dmntk-model"               10
  publish "recognizer"        "dmntk-recognizer"          11
  publish "model-evaluator"   "dmntk-model-evaluator"     12
  publish "evaluator"         "dmntk-evaluator"           13
  publish "gendoc"            "dmntk-gendoc"              14
  publish "workspace"         "dmntk-workspace"           15
  publish "server"            "dmntk-server"              16
  publish "dmntk"             "dmntk"                     17
}

publish_all

cargo +nightly fmt

# dmntk-common = { path = "./common" }
# dmntk-evaluator = { path = "./evaluator" }
# dmntk-examples = { path = "./examples" }
# dmntk-feel = { path = "./feel" }
# dmntk-feel-evaluator = { path = "./feel-evaluator" }
# dmntk-feel-grammar = { path = "./feel-grammar" }
# dmntk-feel-number = { path = "./feel-number" }
# dmntk-feel-parser = { path = "./feel-parser" }
# dmntk-feel-temporal = { path = "./feel-temporal" }
# dmntk-gendoc = { path = "./gendoc" }
# dmntk-macros = { path = "./macros" }
# dmntk-model = { path = "./model" }
# dmntk-model-evaluator = { path = "./model-evaluator" }
# dmntk-recognizer = { path = "./recognizer" }
# dmntk-server = { path = "./server" }
# dmntk-workspace = { path = "./workspace" }
