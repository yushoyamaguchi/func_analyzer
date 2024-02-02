# How to Use
```
cargo run -- <config_file>  [<output_file>]
```
```output_file``` is optional. Default is "yaml_output/call_graph.yaml".
```output_file``` is generated in the directory "yaml_output" .
```config_file``` 's path is relative to the root of the project.

# config_file
```
C source files (relative to the root of the project : comma separated)
Target function name
Depth of the call graph
```

## Example
```
c_src_files/example.c , c_src_files/example2.c
fnA
3
```

# Caution
C source code must be written in a specific format like Linux kernel source code.
For example,
```c
if (condition) { // a space after "if"/"for"/"while" is required
    statement;
}
```

```c
funcA()
{  // "{" must be in the next line of the defined function name
    statement;
}
```

```c
/*
* Comment line in multiple lines comment out must be start with "*"
*/

```