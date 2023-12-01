


name2ch = {
    'one' : '1',
    'two' : '2',
    'three' : '3',
    'four' : '4',
    'five' : '5',
    'six' : '6',
    'seven' : '7',
    'eight' : '8',
    'nine' : '9',
}

with open("../input.txt", 'r') as inf:

    sum = 0

    for line in inf:
        first = None
        last = None

        line = line.strip()
        if not line:
            continue

        for i in range(len(line)):
            c = line[i]
            for name, ch in name2ch.items():
                if line[i:].startswith(name):
                    c = ch
                    break
            if c.isdigit():
                last = c
                if first is None:
                    first = last       

        
        sum += int(first + last)
        
    print(sum)