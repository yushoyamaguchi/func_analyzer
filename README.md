# What is this?
This is a tool to generate a call graph of C source code.
<br>
caller relation or callee relation can be selected.
<br>
Since we are only analyzing the text of the source code, there may be omissions or mistakes.

# How to Use
```
cargo run -- <config_file>  [<output_file>]
```
```output_file``` is optional. Default is "yaml_output/call_graph.yaml".
<br>
If you designate the outputed yaml file name, ```output_file``` is generated in the directory "/yaml_output" .
<br>
```config_file``` 's path is relative to the root of the project.

## Example
```
cargo run -- ex_config_files/example_config.txt ex_callee.yaml
```

# config_file
```
C source files (relative to the root of the project : comma separated)
Target function name
Depth of the call graph
mode (callee or caller)
```

## Example
```
c_src_files/example.c , c_src_files/example2.c
fnA
3
callee
```

# Output
When we use [this](./ex_c_src_files/example.c) as a source code, and [this](./ex_config_files/example_conf1.txt) as a config file, and execute the command below,
```
cargo run -- ex_config_files/example_conf1.txt
```
we get [this](./ex_yaml_output/example_callee.yaml) as a result.

```yaml
<callee_graph>
0: fnA()
    1: printf()
    1: fnB()
        2: printf()
    1: fnC()
        2: printf()
        2: fnD()
            3: printf()

```

# Caution
C source code must be written in a specific format like Linux kernel source code.
<br>
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
* Comment line in multiple lines comment out must start with "*"
*/

```