import {readFileSync, promises as fsPromise} from 'fs';

function syncReadFile(filename) {
    const contents = readFileSync(filename, 'utf-8');

    const arr = contents.split(/\r?\n/);
    
    return arr;
}

const codes = syncReadFile('input.txt');
// const codes = syncReadFile('input_t.txt');

function checkContains(codes) {
    let count = 0;

    for (let line of codes) {
        if (line === '') continue;

        const [first, second] = line.split(',');

        const [firstId1, firstId2] = first.split('-');
        const id1 = parseInt(firstId1);
        const id2 = parseInt(firstId2);
        const [secondId1, secondId2] = second.split('-');
        const sid1 = parseInt(secondId1);
        const sid2 = parseInt(secondId2);
        
        let arr1 = [], arr2 = [];

        for (let i = id1; i <= id2; i++) {
            arr1.push(i);
        }

        for (let i = sid1; i <= sid2; i++) {
            arr2.push(i);
        }

        const intersection = arr1.filter(x => arr2.includes(x));

        if (intersection.length > 0) {
            count++;
        }
    }
    return count;
}

console.log(checkContains(codes));
