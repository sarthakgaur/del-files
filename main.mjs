import { opendir } from 'fs/promises';

// TODO List the directory supplied by the user. Done.

async function listDirectory(path) {
  try {
    const dir = await opendir(path);
    for await (const dirent of dir) {
      console.log(dirent.name);
    }
  } catch (err) {
    console.error(err);
  }
}

function getPath() {
  const path = process.argv[2];
  if (!path) {
    console.error('Error: Path not provided.');
    process.exit(1);
  }
  return path;
}

const path = getPath();
listDirectory(path);
