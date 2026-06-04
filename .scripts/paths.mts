import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const filename = fileURLToPath(import.meta.url);
export const dirname = path.dirname(filename); 
export const root = path.normalize(path.join(dirname, "..")); 

export const Paths = Object.freeze({
  ['~/root']: root,
  ['~/root/']: (...args: string[]) => path.normalize(path.join(root, ...args)),
  ['~/root/static']: path.join(root, "client"),
  ['~/root/static/']: (...args: string[]) =>  path.normalize(path.join(root, "client", ...args)),
  ['~/root/dist']: path.join(root, "dist"),
  ['~/root/dist/']: (...args: string[]) =>  path.normalize(path.join(root, "dist", ...args)),
})
