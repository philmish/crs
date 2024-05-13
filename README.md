## crs - continuously run script

crs - command line file watcher

Usage: crs <file> <command> [args...]

crs is a command line filewatcher which runs the specified
<command> with the provided [args...] whenever a modification
of the file is detected.

Example:
```shell
crs src/main.rs cargo test
```


