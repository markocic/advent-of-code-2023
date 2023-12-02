sum = 0
first = 0
last = 0
isFirst = True

mappings = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9
}
def contains_num(line: str) -> int:
    print(line)
    for key in mappings.keys():
        if key in line:
            return mappings.get(key) or 0

    return 0

def find_first(line: str) -> int:
    curr = ""
    for char in line:
        if char.isnumeric():
            return int(char)
        
        curr += char
        num = contains_num(curr)
        if num != 0:
            return num
    
    return 0

def find_last(line: str) -> int:
    curr = ""
    for char in line[::-1]:
        if char.isnumeric():
            return int(char)
        
        curr += char
        num = contains_num(curr[::-1])
        if num != 0:
            return num
    
    return 0

with open("input.txt", "r") as file:
    for line in file:
        first = find_first(line)
        last = find_last(line)

        # print(f"{line}: {first * 10 + last}")
        sum += first * 10 + last

print(sum)

