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

        if (id1 <= sid1 && sid2 <= id2) {
            count++;
        } else if (sid1 <= id1 && id2 <= sid2) {
            count++;
        }

    }
    return count;
}

console.log(checkContains(codes));
