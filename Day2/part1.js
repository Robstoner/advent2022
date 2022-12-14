import {readFileSync, promises as fsPromise} from 'fs';

function syncReadFile(filename) {
    const contents = readFileSync(filename, 'utf-8');

    const arr = contents.split(/\r?\n/);
    
    return arr;
}

// const guide = syncReadFile('input_t.txt');
const guide = syncReadFile('input.txt');

// console.log(guide);

function calculatePoints(guide) {
    const shapeDict = {
        'X': 1,
        'Y': 2,
        'Z': 3,
    }
    
    const resultDict = {
        'win': 6,
        'draw': 3,
        'lose': 0,
    }

    const typeDict = {
        'A' : 'X',
        'B' : 'Y',
        'C' : 'Z',
    }

    let points = 0;

    for (let line of guide) {
        if (line == '') continue;
        const [enemy, me] = line.split(' ');
        console.log(enemy, me, points)
        if ((me == 'X' && enemy == 'C') || (me == 'Y' && enemy == 'A') || (me == 'Z' && enemy == 'B')) {
            points += resultDict['win'] + shapeDict[me];
        }
        else if (typeDict[enemy] == me) {
            points += resultDict['draw'] + shapeDict[me];
        }
        else {
            points += resultDict['lose'] + shapeDict[me];
        }
    }
    return points;
}

console.log(calculatePoints(guide));

