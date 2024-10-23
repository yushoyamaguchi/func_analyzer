# cflow
## callee analysis
```
cflow -m <target-function> --depth <N> <fileA> <fileB> <fileC>
```

## caller analysis
target functionは指定できない
```
cflow --depth <N> -r <fileA> <fileB> <fileC>
```

# Rust version を作るとしたら
rust-analyzerのAPIを使って自作するのが良さそう