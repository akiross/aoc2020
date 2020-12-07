def calcola(riga):
    row = int('0b' + riga[:7], 2)
    col = int('0b' + riga[7:], 2)
    return row * 8 + col

lines = []
for line in open("src/input", "rt"):
    line = line.strip().replace("F", "0").replace("B", "1").replace("L", "0").replace("R", "1")
    lines.append(line)
lines = list(sorted(lines))

print(lines[0])
primo = calcola(lines[0])

print(lines[-1])
ultimo = calcola(lines[-1])

for i, ticket in enumerate(lines, primo):
    print(ticket, calcola(ticket), i)
    if calcola(ticket) != i:
        print("posto", ticket, calcola(ticket))
        break
