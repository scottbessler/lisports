import { existsSync } from "node:fs";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";

export const fetchFromCache = async (key: string) => {
  try {
    const filename = toFileName(key);
    if (!existsSync(filename)) {
      return undefined;
    }
    const result = await readFile(filename, { encoding: "utf-8" });
    return JSON.parse(result);
  } catch (err) {
    console.error(err);
    return undefined;
  }
};

const dataPath = process.env.DATA_PATH || "data";
function toFileName(key: string) {
  return path.join(dataPath, `${key}.json`);
}

export const saveToCache = async (key: string, value: any) => {
  console.log(`storing cache to ${toFileName(key)}`);
  await writeFile(toFileName(key), JSON.stringify(value), {
    encoding: "utf-8",
  });
};
