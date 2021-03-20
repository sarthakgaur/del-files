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
// TODO Add list of file/directory names to exclude from search.

const rl = readLine.createInterface({
  input: process.stdin,
  output: process.stdout
});

function parseArgs(args) {
  const config = {
    targets: new Set()
  };

  for (let i = 0; i < args.length; i++) {
    const arg = args[i];

    if (arg[0] === '-') {
      for (const char of arg) {
        if (char === 'r') {
          config.recurse = true;
        } else if (char === 'y') {
          config.skipConfirmation = true;
        } else if (char === 'd') {
          const nextArg = args[i + 1];
          if (nextArg && nextArg[0] !== '-') {
            config.path = nextArg;
            i++;
            break;
          }
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

  return config;
}

async function getDirectoryList(path) {
  const directoryList = [];
  const dir = await fs.opendir(path);
  for await (const dirent of dir) {
    directoryList.push(dirent);
  }
  return directoryList;
}

async function removeTargetObject(path) {
  try {
    await fs.rm(path, { recursive: true });
    console.log(`File/Directory ${path} deleted successfully`);
  } catch (e) {
    console.error(`Failed to delete file/directory ${path}. Reason ${e}`);
  }
}

function handleTargetRemoval(config, path) {
  if (config.skipConfirmation) {
    return removeTargetObject(path);
  }

  return new Promise((resolve) => {
    rl.question(`Remove file/directory ${path} [y/n]? `, async (input) => {
      if (input === 'y') {
        await removeTargetObject(path);
      } else {
        console.log(`Skipping file/directory ${path}`);
      }
      resolve();
    });
  });
}

async function main() {
  try {
    const config = parseArgs(process.argv.slice(2));
    const paths = [config.path];
    const targetPaths = [];

    for (let i = 0; i < paths.length; i++) {
      const directoryList = await getDirectoryList(paths[i]);

      for (const dirent of directoryList) {
        const fullPath = path.join(paths[i], dirent.name);

        if (config.targets.has(dirent.name)) {
          targetPaths.push(fullPath);
        }

        if (dirent.isDirectory()
          && config.recurse
          && !config.targets.has(dirent.name)) {
          paths.push(fullPath);
        }
      }
    }

    for (const targetPath of targetPaths) {
      await handleTargetRemoval(config, targetPath);
    }

    if (!targetPaths.length) {
      console.log('Target file/directory not found.');
    }
  } catch (err) {
    console.error(err);
  } finally {
    rl.close();
  }
}

main();
