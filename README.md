# Jbdfs

Jbdfs (June big data file system) is a file storage format which is effectively a filesystem within a couple large files.

This is super useful for storing a lot of small files!


## Usage

The attached binary (See the releases tab) is used to read/write to this file.

Usage:

```
jbdfs <path to filesystem> <operation> <filesystem path> <extra argument>
```

The operation is a 2-character id for each operation;

The first character is either a 1 or a 0 - A file, or a directory. The second character is between 0-9; it's what you want to do to this file/directory, here's a list of all of them;

```
0 - Read file / Return files in directory   Files + Directories    No extra argument
1 - Unused                                
2 - Get file type                           Files only             No extra argument
3 - Write to file                           Files only             Requires an extra argument - Data to write to file
4 - Create file                             Files + Directories    Requires an extra argument - Filetype, files only
5 - Remove file                             Files + Directories    No extra argument
6 - Unused
7 - Unused
8 - Unused
9 - Unused
```

For example, if you wanted to read a file, you'd use `10`, if you wanted to create a file, you'd use `14`, if you wanted to create a directory instead, you would use `04`.

to create/remove a new jbdfs filesystem, there's two commands;

Create a new filesystem at `/home/june/filesystem`:

```
jbdfs cfs /home/june/filesystem
```
Remove the filesystem at `/home/june/filesystem`:

```
jbdfs rfs /home/june/filesystem
```

The filepath syntax for files within the filesystem is very different to a normal filesystem.

```
root -> somefolder -> somefile
```
It uses arrows instead of normal `/` or `\`. The above can also be written in a few different ways;
```
root->somefolder->somefile
somefolder -> somefile
somefolder->somefile
```
spaces cannot be used in file/directory names and will be automatically removed.

## Extra info

- Currently WIP
- Currently very slow
- Unfinished
