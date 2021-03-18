import { opendir, lstat } from 'fs/promises';
import Path from 'path';

// TODO List the directory supplied by the user. Done.
// TODO If node_modules directory is found, notify the user. Done.

function getPath() {
  const path = process.argv[2];
  if (!path) {
    console.error('Error: Path not provided.');
    process.exit(1);
  }
  return path;
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
    const path = getPath();
    const directoryList = await getDirectoryList(path);
    console.log(directoryList);

    for (const dirent of directoryList) {
      if (await isNodeModules(path, dirent.name)) {
        console.log(`node_modules directory found: ${Path.join(path, dirent.name)}`);
      }
    }
  } catch (err) {
    console.error(err);
  }
}

main();
