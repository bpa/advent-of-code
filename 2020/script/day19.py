#!/usr/bin/env python3

input = "../input/2020/day19.txt"
# input = "test.txt"
matchers = {}


def match_char(substr):
    size = len(substr)

    def _match(input, input_size, i):
        end = i + size
        if end > input_size:
            return False, i
        return input[i:end] == substr, end

    return _match


def match_all(required):
    def _match(input, input_size, i):
        for req in required:
            f = matchers[req]
            matched, i = f(input, input_size, i)
            if not matched:
                return False, i
        return True, i
    return _match


def match_one(options):
    def _match(input, input_size, i):
        for o in options:
            matched, end = o(input, input_size, i)
            if matched:
                return matched, end
        return False, i
    return _match


def special_match(input, input_size, i):
    leading = matchers[42]
    trailing = matchers[31]

    first = -1
    matched = True
    while matched:
        first += 1
        matched, i = leading(input, input_size, i)

    second = -1
    matched = True
    while matched:
        second += 1
        matched, i = trailing(input, input_size, i)

    return first > second > 0, i


with open(input) as file:
    while True:
        line = file.readline()
        if line.isspace():
            break
        parts = line.strip().split(' ')
        id = int(parts[0][:-1])
        if parts[1].startswith('"'):
            matchers[id] = match_char(parts[1][1:-1])
        else:
            options = []
            rule = []
            for req in parts[1:]:
                if req == '|':
                    options.append(match_all(rule))
                    rule = []
                else:
                    rule.append(int(req))
            options.append(match_all(rule))
            matchers[id] = match_one(options)

    answer = 0
    while line := file.readline().strip():
        matched, chars = matchers[0](line, len(line), 0)
        if matched and chars == len(line):
            answer += 1
    print("Part 1:", answer)

matchers[0] = special_match

with open(input) as file:
    while not file.readline().isspace():
        pass

    answer = 0
    while line := file.readline().strip():
        matched, chars = matchers[0](line, len(line), 0)
        if matched and chars == len(line):
            # print("Yes", line)
            answer += 1
        # else:
        #     print("No!", chars, len(line), line)
    print("Part 2:", answer)
