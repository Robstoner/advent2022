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

    let i = 0;
    while (i < backpacks.length) {
        const first = backpacks[i];
        const second = backpacks[i + 1];
        const third = backpacks[i + 2];
        
        for (let j = 0; j < first.length; ++j) {
            if (second.includes(first[j]) && third.includes(first[j])) {
                sum += dict[first[j]];
                break;
            }
        }
        
        i+=3;
    }

    return sum;
}

console.log(calculateSum(backpacks));
