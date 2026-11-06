import sys
import os
import io


def preprocess(src: str) -> str:
    out = io.StringIO()

    comment = None
    for i, c in enumerate(src):
        if comment is None:
            if c == '/' and i + 1 < len(src) and src[i+1] == '*':
                comment = 'multi'
            elif c == '/' and i > 0 and src[i-1] == '/':
                comment = 'single'
            elif c == '#':
                comment = 'single'
            else:
                out.write(c)
        elif comment == 'single':
            if c == '\n' and src[i-1] != '\\':
                comment = None
                out.write('\n')
        elif comment == 'multi':
            if c == '/' and src[i - 1] == '*':
                comment = None
        else:
            raise AssertionError(comment)

    return out.getvalue()


if len(sys.argv) != 2:
    print(f'Usage: {sys.argv[0]} SDL3_installation_prefix', file=sys.stderr)

print(f'// Generated using: python3 {" ".join(sys.argv)}')
print('#![allow(unused)]')
print()

prefix = sys.argv[1]

with open(os.path.join(prefix, 'SDL_scancode.h')) as file:
    lines = preprocess(file.read()).split('\n')

while lines:
    line = lines.pop(0).strip()
    if line == 'typedef enum SDL_Scancode':
        break

for line in lines:
    line = line.strip()
    name, sep, val = line.partition(' = ')
    if not sep:
          continue

    assert name.startswith('SDL_')
    name = name.removeprefix('SDL_')

    val = val.removesuffix(',')
    val = int(val)

    print(f'pub const {name}: u32 = {val};')

