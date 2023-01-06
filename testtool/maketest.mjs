import fs from "fs";
import http from "http";
import process from "process";
import SamJs from "./sam-js/src/index.js";

import collect, { result } from "./collect.mjs";

// Take port from command line argument
const port = parseInt(process.argv[process.argv.length - 1]);

// Set up HTTP server
const server = http.createServer(function(request, response) {
    let data = "";

    request.on("data", function(chunk) {
        data += chunk;
    });

    request.on("end", function() {
        const input = JSON.parse(data);

        Object.keys(result).forEach(function(key) {
            delete result[key];
        });

        const sam = new SamJs({
            phonetic: input.phonetic,
            singmode: input.sing_mode,
            debug: false,
            pitch: input.pitch,
            speed: input.speed,
            mouth: input.mouth,
            throat: input.throat
        });

        collect("input", input);

        const phonemes = SamJs.convert(input.text);
        collect("recited", phonemes);

        const output = sam.buf8(input.text);
        collect("rendered", Object.values(output));

        response.writeHead(200);
        response.end(JSON.stringify(result));
    });
});

// Signal to caller that server is ready
setTimeout(function() {
    console.log("ready");
}, 250);

server.listen(port);

//const input = JSON.parse(fs.readFileSync(0, "utf-8"));

//const sam = new SamJs({
    //phonetic: input.phonetic,
    //singmode: input.sing_mode,
    //debug: false,
    //pitch: input.pitch,
    //speed: input.speed,
    //mouth: input.mouth,
    //throat: input.throat
//});

//collect("input", input);

//const phonemes = SamJs.convert(input.text);
//collect("recited", phonemes);

//const output = sam.buf8(phonemes);
//collect("rendered", Object.values(output));

//console.log(JSON.stringify(result));
//console.log(result);
