import { readFileSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
export const TEST_PRIV_KEY = new Uint8Array(readFileSync(resolve(__dirname, 'keys/priv.bin')));
export const TEST_PUB_KEY = new Uint8Array(readFileSync(resolve(__dirname, 'keys/pub.bin')));
