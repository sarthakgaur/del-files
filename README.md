# del-files
Delete files and directories from the command line.

## Installation
1. `git clone https://github.com/sarthakgaur/del-files`
2. `cd del-files`
3. `cargo build --release`
4. `The executable is located in target/release`

## Command Line Arguments
    USAGE:
        del-files [FLAGS] [OPTIONS] --directory <directory> --targets <FILE>...

    FLAGS:
        -h, --help                 Prints help information
        -r, --recurse              Search recursively for files/directories.
        -s, --size                 Output the disk space freed.
        -y, --skip-confirmation    Skip confirming when deleting file/directory.
        -V, --version              Prints version information

    OPTIONS:
        -d, --directory <directory>                          Specify the directory to search in.
        -e, --exlude-directories <exclude_directories>...    Specify the directory to exclude from search.
        -t, --targets <FILE>...                              Files or directories to delete.

## Example Usage

Command: `node main.mjs node_modules package-lock.json -d ../local_chat/ -r -e .git public env`

This command will search for `node_modules` and `package-lock.json` in the `local_chat` directory recursively, and then delete them after confirming from the user. The program will exclude `.git`, `public`, and `env` directories from the search.
