#!/bin/bash

# テストを実行する関数
run_test() {
  local INPUT_FILE=$1
  local EXPECTED_OUTPUT_FILE=$2

  # cargo run の実行
  cargo run -- "$INPUT_FILE"

  # 実際の出力をファイルから読み込む
  local OUTPUT_FILE="yaml_output/callee_graph.yaml"
  local OUTPUT=$(cat "$OUTPUT_FILE")

  # 期待される出力をファイルから読み込む
  local EXPECTED_OUTPUT=$(cat "$EXPECTED_OUTPUT_FILE")

  # 出力を比較
  if [[ "$OUTPUT" == "$EXPECTED_OUTPUT" ]]; then
    echo "✅ Test passed for input file: $INPUT_FILE"
    return 0
  else
    echo "❌ Test failed for input file: $INPUT_FILE"
    return 1
  fi
}

# テストを実行する
run_test "ex_config_files/example_conf1.txt" "output_for_test/example_callee.yaml"

