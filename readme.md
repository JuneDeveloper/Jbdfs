# Jbdfs

Jbdfs (June big data filesystem) is a way to store large amounts of data in one place.

It is effectively a filesystem within a file - You have folders and files like you would with any other filesystem, but what/how you use and store your data is entirely up to you.

> **Warning**: This software is still pretty experimental, expect big, possibly breaking changes, and not great performance/features!

## Usage

Using it is pretty simple. You can create a new filesystem with;
```rust
jbdfs -cfs "/path/to/location"
```
and remove it with;
```rust
jbdfs -rfs "/path/to/location"
```
Accessing data within the filesystem is very different however, but it is pretty simple to understand.

```rust
jbdfs <action> <path> <filepath> <extras>
```

Specifying an action is the most important step to accessing the data within. It's simply 2 numbers between 0-9. The first number is to specify the type of file you want to access, either a 0 ***(Directory)*** or a 1 ***(File)***. The second number is to sepcify the action you want to perform (Create, read, write, etc). Here's a table with all of the possible types:

```
  Action ID:                     Action Behaviour:		     	        Applies to:
      0                               Create                                 Both
      1                               Remove                                 Both
      2                                Read                                  Both (Returns subfiles with folders, file data with files)
      3                               Write                                Files Only
      4                               Unused                                 N/A
      5                               Unused                                 N/A
      6                               Unused                                 N/A
      7                               Unused                                 N/A
      8                               Unused                                 N/A
      9                               Unused                                 N/A   
```
So if you want to create a folder, you'd use;
```rust
jbdfs 00 /path/to/filesystem "path -> to -> folder"
```
For a file;
```rust
jbdfs 10 /path/to/filesystem "path -> to -> folder"
```

For an example, this set of commands will create a directory with a file inside, write "Hello, world!" to the file and finally read it:

```rust
// Create directory "hello"
jbdfs 00 /home/june/fs "root -> hello"
// Create file "world" inside the newly created "hello" directory
jbdfs 10 /home/june/fs "root -> hello -> world", "txt"
// Write "Hello, world!" to that file
jbdfs 13 /home/june/fs "root -> hello -> world" "Hello, world!"
// Read file
jbdfs 12 /home/june.fs "root -> hello -> world"
```

As you may have noticed, the filepaths are different to how a filesystem will normally display it. There's a few different options;

```rust
root -> folder -> file
root->folder->file
root > folder > file
root>folder>file
root/folder/file
root\folder\file
```

All of these are valid, it's up to personal preference which one you want to use.

