def instr_exists(instr, instr_num):
    return instr in instr_num

def indent(depth):
    return "    " * (depth + 1)

def prefix_exists(prefix, instructions):
    return any([instr.startswith(prefix) for instr in instructions])

with open("instructions.txt", "r") as fd:
    instructions_with_index = [(instr.strip(), i) for (i, instr) in enumerate(fd.readlines())]

instructions_with_index.sort()

curr_instr = ""
instructions = [instr for (instr, _) in instructions_with_index]
potential_chars = ""
instr_num = {}
instr_index = {}

code = "pub fn instr_search(instr: &str) -> Option<u16> {\n"

for (j, instr_i) in enumerate(instructions_with_index):
    instr, i = instr_i
    instr_num[instr] = i
    instr_index[instr] = j
    for char in instr:
        if char not in potential_chars:
            potential_chars += char

def process_depth(depth):
    global code, curr_instr, instr_index, instr_num, potential_chars, instructions
    ind = indent(depth);
    code += ind
    if instr_exists(curr_instr, instr_num):
        code += f"if instr.len() < {depth + 1} {{ return Some({instr_num[curr_instr]}); }}\n"
    else:
        code += f"if instr.len() < {depth + 1} {{ return None; }}\n"

    for letter in potential_chars:
        if prefix_exists(curr_instr + letter, instructions):
            code += ind + f"if instr.as_bytes()[{depth}] == b'{letter}' {{\n"
            curr_instr += letter
            process_depth(depth + 1)
            curr_instr = curr_instr[:-1]
            code += ind + "}\n"

process_depth(0)
code += indent(0) + "return None;\n"
code += "}"

with open("src/generated.rs", "w") as fd:
    fd.write(code)
