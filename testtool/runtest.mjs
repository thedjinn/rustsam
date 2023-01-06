import SamJs from "./sam-js/src/index.js";

import collect, { result } from "./collect.mjs";

const text = "FC ";

const sam = new SamJs({
    phonetic: false,
    singmode: false,
    debug: false,
    pitch: 64,
    speed: 72,
    //mouth: 64,
    //throat: 96
    mouth: 128,
    throat: 128
});

const phonemes = SamJs.convert(text);
collect("recited", phonemes);

const output = sam.buf8(text);
collect("rendered", Object.values(output));

console.log(result);
