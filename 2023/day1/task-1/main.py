



with open("../input.txt", 'r') as inf:

    sum = 0

    for line in inf:
        first = None
        last = None

        line = line.strip()
        if not line:
            continue

        for c in line:
            if c.isdigit():
                last = c
                if first is None:
                    first = c
        
        sum += int(first + last)
        
    print(sum)