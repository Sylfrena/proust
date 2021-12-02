# proust
Rust library for procfs

A Rust library for fetching information from the proc filesystem at `/proc`.

Currently, `proust` supports the following procfs interfaces:

- __/proc/cpuinfo__ : ```module cpuinfo``` 
- __/proc/\<pid>/status__ : `module proc_status`
- __/proc/meminfo__ : `module meminfo`

**TODO**

- Add remaining proc_status fields.
- Make crate publishable
- Add parser for diff field types

