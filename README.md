# del-files
Delete files and directories from the command line.

## Installation
1. `git clone https://github.com/sarthakgaur/del-files`
2. `node main.mjs`

## Command Line Arguments
1. `-d`: Specify the directory to search in. If no directory is specified, the `home` directory is used.
2. `-y`: Skip the confirmation when deleting the file/directory.
3. `-r`: Search recursively for files/directories.

## Example Usage

Command: `node main.mjs node_modules package-lock.json -d ../local_chat/ -r`

This command will search for `node_modules` and `package-lock.json` in the `local_chat` directory recursively, and then delete them after confirming from the user.
