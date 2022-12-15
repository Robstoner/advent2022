import re

f = open("input.txt", "r")
# f = open("input_t.txt", "r")

input = f.read().splitlines()


def calculateDirSizes(input):
    dirSizes = {}
    for line in input:
        if (len(line) > 0):
            if (line[0] == '$'):
                if (line[3] == 'c'):
                    dirName = line[5:]
                    dirSizes[dirName] = 0
                if (line[3] == 'l'):
                    dirSizes