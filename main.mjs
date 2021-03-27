import fs from 'fs/promises';
import readLine from 'readline';
import os from 'os';
import path from 'path';

// TODO List the directory supplied by the user. Done.
// TODO If node_modules directory is found, notify the user. Done.
// TODO Parse the command line arguments. Done.
// TODO Add option to recursively search directories. Done.
// TODO Prompt the user before deleting the directory. Done.
// TODO Add support for removing any directory or file. Done.
// TODO Add support for removing multiple directories or files. Done.
// TODO Add list of file/directory names to exclude from search. Done.
// TODO Ouput storage space freed. Done.

const rl = readLine.createInterface({
  input: process.stdin,
  output: process.stdout
});

function parseArgs(args) {
  const config = {
    targets: new Set(),
    excludeList: new Set()
  };

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    if (arg[0] === '-') {
      for (const char of arg) {
        if (char === 'r') {
          config.recurse = true;
        } else if (char === 'y') {
          config.skipConfirmation = true;
        } else if (char === 's') {
          config.getSize = true;
        } else if (char === 'd') {
          const nextArg = args[i + 1];
          if (nextArg && nextArg[0] !== '-') {
            config.path = nextArg;
            i++;
            break;
          }
        } else if (char === 'e') {
          let nextArg = args[++i];
          while (nextArg && nextArg[0] !== '-') {
            config.excludeList.add(nextArg);
            nextArg = args[++i];
          }
          i--;
          break;
        }
      }
    } else {
      config.targets.add(arg);
    }
  }

  if (!config.path) {
    config.path = os.homedir();
  }

  if (!config.targets.size) {
    throw new Error('Target file/directory not provided.')
  }

  config.targets.forEach(config.excludeList.add, config.excludeList);

  return config;
}

async function getAllFilesPath(dirPath) {
  const paths = [dirPath];
  const filePaths = [];

  if ((await fs.stat(dirPath)).isFile()) {
    return [dirPath];
  }

  for (let i = 0; i < paths.length; i++) {
    const directoryList = await fs.opendir(paths[i]);

    for await (const dirent of directoryList) {
      const fullPath = path.join(paths[i], dirent.name);

      if (dirent.isFile()) {
        filePaths.push(fullPath);
      } else {
        paths.push(fullPath);
      }
    }
  }

  return filePaths;
}

async function getDirSize(dirPath) {
  let totalSize = 0;
  const filePaths = await getAllFilesPath(dirPath);

  for (const filePath of filePaths) {
    totalSize += (await fs.stat(filePath)).size;
  }

  return totalSize;
}

function convertBytes(bytes) {
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];

  if (bytes == 0) {
    return "0 Bytes";
  }

  const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
  return (bytes / Math.pow(1024, i)).toFixed(1) + " " + sizes[i];
}

async function removeTargetObject(path) {
  try {
    await fs.rm(path, { recursive: true });
    console.log(`File/Directory ${path} deleted successfully.`);
    return true;
  } catch (e) {
    console.error(`Failed to delete file/directory ${path}. Reason ${e}`);
    return false;
  }
}

async function handleTargetRemoval(getSize, skipConfirmation, path) {
  const operation = {};

  if (skipConfirmation) {
    operation.size = getSize ? await getDirSize(path) : 0;
    operation.isSuccess = await removeTargetObject(path);
    return operation;
  }

  return new Promise((resolve) => {
    rl.question(`Remove file/directory ${path} [y/n]? `, async (input) => {
      if (input === 'y') {
        operation.size = getSize ? await getDirSize(path) : 0;
        operation.isSuccess = await removeTargetObject(path);
      } else {
        console.log(`Skipping file/directory ${path}`);
      }
      resolve(operation);
    });
  });
}

async function main() {
  try {
    const config = parseArgs(process.argv.slice(2));
    const paths = [config.path];
    const targetObjects = [];
    let totalSize = 0;

    console.log('Searching for files/directories...');

    for (let i = 0; i < paths.length; i++) {
      const directoryList = await fs.opendir(paths[i]);

      for await (const dirent of directoryList) {
        const fullPath = path.join(paths[i], dirent.name);

        if (config.targets.has(dirent.name)) {
          targetObjects.push({ path: fullPath });
        }

        if (dirent.isDirectory()
          && config.recurse
          && !config.excludeList.has(dirent.name)) {
          paths.push(fullPath);
        }
      }
    }

    if (targetObjects.length) {
      console.log(`${targetObjects.length} target(s) found.`);
    } else {
      console.log('Target file/directory not found.');
    }

    for (const targetObject of targetObjects) {
      const operation = await handleTargetRemoval(
        config.getSize, config.skipConfirmation, targetObject.path
      );
      totalSize += operation.size ? operation.size : 0;
    }

    if (config.getSize) {
      console.log(`${convertBytes(totalSize)} freed.`)
    }
  } catch (err) {
    console.error(err);
  } finally {
    rl.close();
  }
}

main();
