import { readFileSync } from 'fs';
import { resolve, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
export const TEST_PRIV_KEY_DER = new Uint8Array(readFileSync(resolve(__dirname, 'keys/priv.der')));
export const TEST_PUB_KEY_DER = new Uint8Array(readFileSync(resolve(__dirname, 'keys/pub.der')));
