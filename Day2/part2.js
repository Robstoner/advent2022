import { readFileSync, promises as fsPromise } from 'fs';

function syncReadFile(filename) {
    const contents = readFileSync(filename, 'utf-8');

    const arr = contents.split(/\r?\n/);

    return arr;
}

// const guide = syncReadFile('input_t.txt');
const guide = syncReadFile('input.txt');

// console.log(guide);


function calculatePoints(guide) {
    let points = 0;

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

    const winTypeDict = {
        'A': 'Y',
        'B': 'Z',
        'C': 'X',
    }

    const loseTypeDict = {
        'A': 'Z',
        'B': 'X',
        'C': 'Y',
    }

    for (let line of guide) {
        
        if (line == '') continue;
        const [enemy, result] = line.split(' ');
        console.log(enemy, result, points)

        switch (result) {
            case 'X': {
                //lose
                points += resultDict['lose'];

                points += shapeDict[loseTypeDict[enemy]];
                
                break;
            }
            case 'Y': {
                //draw
                points += resultDict['draw'];
                
                points += shapeDict[typeDict[enemy]]

                break;
            }
            case 'Z': {
                //win
                points += resultDict['win'];

                points += shapeDict[winTypeDict[enemy]];

                break;
            }
        }
    }
    return points;
}

console.log(calculatePoints(guide));