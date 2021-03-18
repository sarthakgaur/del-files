import { opendir, lstat } from 'fs/promises';
import Path from 'path';

// TODO List the directory supplied by the user. Done.
// TODO If node_modules directory is found, notify the user. Done.
// TODO Parse the command line arguments. Done.
// TODO Add option to recursively search directories.

function parseArgs(args) {
  const config = {};

  for (const arg of args) {
    if (arg[0] === '-') {
      for (const char of arg) {
        if (char === 'r') {
          config.recurse = true;
        } else if (char === 'c') {
          config.confirm = true;
        }
      }
    } else if (!config.path) {
      config.path = arg;
    } else {
      throw new Error('Invalid arguments supplied.')
    }
  }

  if (!config.path) {
    throw new Error('Path not provided.');
  }

  return config;
}

async function getDirectoryList(path) {
  const directoryList = [];
  const dir = await opendir(path);
  for await (const dirent of dir) {
    directoryList.push(dirent);
  }
  return directoryList;
}

async function isNodeModules(path, name) {
  const fullPath = Path.join(path, name);
  const stat = await lstat(path);
  return stat.isDirectory() && name === 'node_modules';
}

async function main() {
  try {
    const config = parseArgs(process.argv.slice(2));
    const directoryList = await getDirectoryList(config.path);
    console.log(config);
    console.log(directoryList);

    for (const dirent of directoryList) {
      if (await isNodeModules(config.path, dirent.name)) {
        console.log(`node_modules directory found: ${Path.join(config.path, dirent.name)}`);
      }
    }
  } catch (err) {
    console.error(err);
  }
}

main();
