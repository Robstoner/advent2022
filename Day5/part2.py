import re

f = open("input.txt", "r")
# f = open("input_t.txt", "r")

input = f.read().splitlines()

# print(input)
# create regex to keep only A-Z and spaces

# def cleanInput(input):
#     cleaninput = []
#     nr = 0
#     while(len(input[nr]) > 0):
#         # print(nr)
#         cleaninput.append(re.sub(']*[[]*', '', input[nr]))
#         # print(input[nr])
#         nr += 1
    
#     return cleaninput

# clean = cleanInput(input)
# print(clean)

def createStacks(input):
    stack = {
        1: [],
        2: [],
        3: [],
        4: [],
        5: [],
        6: [],
        7: [],
        8: [],
        9: [],
    }
    nr = 0
    while(len(input[nr]) > 0):
        # stack[1].append(input[nr][1]) and ((len(input[nr]) >=3) and (input[nr][1] != ' '))
        if (len(input[nr]) >=3 and re.match('[A-Z]', input[nr][1])):
            stack[1].append(input[nr][1])
        # stack[2].append(input[nr][5]) and (len(input[nr]) >=7 and input[nr][5] != ' ')
        if (len(input[nr]) >=7 and re.match('[A-Z]', input[nr][5])):
            stack[2].append(input[nr][5])
        # stack[3].append(input[nr][9]) and (len(input[nr]) >=11 and input[nr][9] != ' ')
        if (len(input[nr]) >=11 and re.match('[A-Z]', input[nr][9])):
            stack[3].append(input[nr][9])
        # stack[4].append(input[nr][13]) and (len(input[nr]) >=15 and input[nr][13] != ' ')
        if (len(input[nr]) >=15 and re.match('[A-Z]', input[nr][13])):
            stack[4].append(input[nr][13])
        # stack[5].append(input[nr][17]) and (len(input[nr]) >=19 and input[nr][17] != ' ')
        if (len(input[nr]) >=19 and re.match('[A-Z]', input[nr][17])):
            stack[5].append(input[nr][17])
        # stack[6].append(input[nr][21]) and (len(input[nr]) >=23 and input[nr][21] != ' ')
        if (len(input[nr]) >=23 and re.match('[A-Z]', input[nr][21])):
            stack[6].append(input[nr][21])
        # stack[7].append(input[nr][25]) and (len(input[nr]) >=27 and input[nr][25] != ' ')
        if (len(input[nr]) >=27 and re.match('[A-Z]', input[nr][25])):
            stack[7].append(input[nr][25])
        # stack[8].append(input[nr][29]) and (len(input[nr]) >=31 and input[nr][29] != ' ')
        if (len(input[nr]) >=31 and re.match('[A-Z]', input[nr][29])):
            stack[8].append(input[nr][29])
        # stack[9].append(input[nr][33]) and (len(input[nr]) >=35 and input[nr][33] != ' ')
        if (len(input[nr]) >=35 and re.match('[A-Z]', input[nr][33])):
            stack[9].append(input[nr][33])
        input[nr] = ''
        nr += 1
    
    stack2 = {
        1: [],
        2: [],
        3: [],
        4: [],
        5: [],
        6: [],
        7: [],
        8: [],
        9: [],
    }
    for i in range(1,10):
        stack2[i] = stack[i][::-1]
    
    return stack2

stack = createStacks(input)

# print(stack)

for i in range(0, len(input)):
    if (len(input[i]) > 0):
        # print(input[i])
        words = input[i].split(' ')
        # print(words)
        x = []
        for i in range(0, int(words[1])):
            x.append(stack[int(words[3])].pop())
        print(x)
        for i in range(0, int(words[1])):
            stack[int(words[5])].append(x.pop())

print(stack)

for i in range(1,10):
    x = stack[i].pop() if len(stack[i]) > 0 else ' '
    print(''.join(x), end='')



