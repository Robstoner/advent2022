import {readFileSync, promises as fsPromise} from 'fs';
import {dict} from './dict.js';

function syncReadFile(filename) {
    const contents = readFileSync(filename, 'utf-8');

    const arr = contents.split(/\r?\n/);
    
    return arr;
}

const backpacks = syncReadFile('input.txt');
// const backpacks = syncReadFile('input_t.txt');

function calculateSum(backpacks) {
    let sum = 0;


    for (let line of backpacks) {
        if (line == '') continue;
        const firstHalf = line.slice(0, line.length / 2);
        const secondHalf = line.slice(line.length / 2, line.length);

        let found = "";

        for (let i = 0; i < firstHalf.length; ++i) {
            //console.log(found.includes(firstHalf[i]))
            if (secondHalf.includes(firstHalf[i]) && !found.includes(firstHalf[i])) {
                sum += dict[firstHalf[i]];
                found += firstHalf[i];
            }
        }
    }

    return sum;
}

console.log(calculateSum(backpacks));