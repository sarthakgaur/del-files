import { opendir, lstat, rm } from 'fs/promises';
import readLine from 'readline';
import path from 'path';

// TODO List the directory supplied by the user. Done.
// TODO If node_modules directory is found, notify the user. Done.
// TODO Parse the command line arguments. Done.
// TODO Add option to recursively search directories. Done.
// TODO Prompt the user before deleting the directory. Done.
// TODO Add support for removing any directory or file.

const rl = readLine.createInterface({
  input: process.stdin,
  output: process.stdout
});

function parseArgs(args) {
  const config = {};

  for (const arg of args) {
    if (arg[0] === '-') {
      for (const char of arg) {
        if (char === 'r') {
          config.recurse = true;
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
  const stat = await lstat(path);
  return stat.isDirectory() && name === 'node_modules';
}

function removeDirectory(path) {
  return new Promise((resolve, reject) => {
    rl.question(`Remove directory: ${path} [y/n]? `, (input) => {
      if (input === 'y') {
        rm(path, { recursive: true })
          .then(() => {
            console.log(`Directory ${path} deleted successfully`);
          }).catch((e) => {
            console.error(`Failed to delete directory ${path}. Reason ${e}`);
          });
      } else {
        console.log(`Skipping directory: ${path}`);
      }
      resolve(undefined);
    });
  });
}

async function main() {
  try {
    const config = parseArgs(process.argv.slice(2));
    const paths = [config.path];
    const dirsToRemove = [];

    for (let i = 0; i < paths.length; i++) {
      const directoryList = await getDirectoryList(paths[i]);

      for (const dirent of directoryList) {
        const fullPath = path.join(paths[i], dirent.name);

        if (await isNodeModules(fullPath, dirent.name)) {
          console.log(`node_modules directory found: ${fullPath}`);
          dirsToRemove.push(fullPath);
        }

        if (dirent.isDirectory()
          && config.recurse
          && dirent.name !== 'node_modules'
          && dirent.name !== '.git') {
          paths.push(fullPath);
        }
      }
    }

    for (const dirToRemove of dirsToRemove) {
      await removeDirectory(dirToRemove);
    }

    rl.close();
  } catch (err) {
    console.error(err);
  }
}

main();
