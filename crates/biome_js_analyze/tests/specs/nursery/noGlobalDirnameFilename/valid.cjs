/* should not generate diagnostics */
const __filename = 1;
const foo = __filename;

const __dirname = 1;
const bar = __dirname;

import {__filename as filename} from "foo.mjs"
const baz = 1;
export {foo as __dirname}