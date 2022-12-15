import re

f = open("input.txt", "r")
# f = open("input_t.txt", "r")

input = f.read()

for i in range(13, len(input)):
    ok = -1
    set = [False] * 128
    for j in range(i - 13, i + 1):
        val = ord(input[j])
        if (set[val] == True):
            ok = 1
            break

        set[val] = True
        
    # print (ok)
    if (ok < 0):
        print(i + 1)
        break
