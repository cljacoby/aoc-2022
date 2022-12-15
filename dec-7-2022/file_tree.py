import sys
from pprint import pprint

class Dir:
    def __init__(self, name):
        self.parent = None
        self.name = name
        self.children = []
    
    def __str__(self):
        return f"Dir {{ name: {self.name}, parent: '{self.parent.name}' len(children): {len(self.children)} }}"
    
    def __repr__(self):
        return str(self)

class File:
    def __init__(self, name, size):
        self.parent = None
        self.name = name
        self.size = int(size)
    
    def __str__(self):
        return f"File {{ name: {self.name}, parent: {self.parent.name}, size: {self.size} }}"

    def __repr__(self):
        return str(self)

# ==============================================================================

def ls(lines, cwd):
    '''As long as we don't see a new command line, pop a line from lines and add
    it as an entry in the current directory.
    '''
    while True:
        if len(lines) == 0 or lines[0].startswith("$"):
            return
        line = lines.pop(0)
        if line.startswith("dir"):
            _dir, name = line.split(" ")[0:2] 
            entry = Dir(name)
            entry.parent = cwd
            cwd.children.append(entry)
        else:
            size, name = line.split(" ")[0:2]
            entry = File(name, size)
            entry.parent = cwd
            cwd.children.append(entry)

def cd(line, cwd):
    '''Change the current working directory, either to a child directory or up a
    directory via '..'
    '''
    dir_name = line[5:]
    if dir_name == "..":
        assert cwd.parent != None , \
            f"Attempt to 'cd ..' but parent of cwd was None, cwd={cwd}"
        return cwd.parent
    next_dir = None
    for entry in cwd.children:
        if not isinstance(entry, Dir):
            continue
        if entry.name == dir_name:
            next_dir = entry
            break
    assert next_dir != None, \
        f"Attempt to cd into dir which wasn't found, cwd={cwd}, line={line}"
    return next_dir

def print_tree(root, indent=0):
    space = "  " * indent
    print(f"{space}dir=[{root.name}]")
    for child in root.children:
        if isinstance(child, Dir):
            print_tree(child, indent+1)
        else:
            print(f"{space}  file=[{child.name} {child.size}]")

def mk_tree(input_path):
    lines = open(input_path).read().strip().split("\n")

    assert lines[0] == "$ cd /", \
        f"First line has to be '$ cd /', lines[0]={lines[0]}"
    lines.pop(0)

    root = Dir("/")
    cwd = root

    while len(lines) > 0:
        line = lines.pop(0)
        if line.startswith("$"):
            cmd = line[2:4]
            match cmd:
                case "ls":
                    ls(lines, cwd)
                case "cd":
                    cwd = cd(line, cwd)
                case _:
                    print(f"error: unrecognized command. cmd={cmd}, line={line}", file=sys.stderr)
                    sys.exit(1)
    return root

# Directory name is not unique throughout input file. Therefore when caching
# directory sizes, use the address of the object as the key
def dir_size(root, cache):
    if id(root) in cache:
        return cache[id(root)]
    size = 0
    for entry in root.children:
        if isinstance(entry, Dir):
            size += dir_size(entry, cache)
        else:
            size += entry.size
    cache[id(root)] = size
    return size


if __name__ == "__main__":
    root = mk_tree("input.txt")
    print_tree(root)
    cache = dict()
    dir_size(root, cache)
    pprint(f"len(cache)={len(cache)}")
    solution = 0
    for (dirname, size) in cache.items():
        if size < 100000:
            print(f"adding dirname={dirname}, size={size}")
            solution += size
    print(f"solution={solution}")