import sys
import re
import numpy as np

def main():
    if len(sys.argv) < 2:
        print("Give input file as arg1")
        return
    file_name = sys.argv[1]
    index_to_name, name_to_index, valves = parse(file_name)
    openable = []
    for i in range(len(valves)):
        if valves[i][0] > 0:
            openable.append(i)
    maxtime = 30
    opt = np.zeros((maxtime+1, len(valves), 1 << len(openable)))
    for time in range(2, maxtime+1):
        for i in range(len(valves)):
            flow, neighbours = valves[i]
            for opened in range(0,1 << len(openable)):
                best = 0
                for neighbour in neighbours:
                    x = opt[time-1][neighbour][opened]
                    if x > best:
                        best = x
                if i in openable and ((1 << openable.index(i)) & opened) == 0:
                    x = (flow * (time-1)) + opt[time-1][i][opened | (1 << openable.index(i))]
                    if x > best:
                        best = x
                opt[time][i][opened] = best

    print(f"Part 1: {opt[30][name_to_index['AA']][0]}")

    best = 0
    for humanValves in range(1 << len(openable)):
        elephantValves = ~humanValves
        score = opt[26][name_to_index["AA"]][humanValves] + opt[26][name_to_index["AA"]][elephantValves]
        if score > best:
            best = score
    print(f"Part 2: {best}")
        
        


def parse(file_name: str) -> tuple[dict[int, str], dict[str, int], list[tuple[int, list[int]]]]:
    valves: dict[str, tuple[int, list[str]]] = dict()
    flowre = re.compile("rate=(\d+)")
    idsre = re.compile("[A-Z][A-Z]")
    with open(file_name) as f:
        for line in f:
            flow = int(re.search(flowre, line).group(1))
            ids = re.findall(idsre, line)
            valves[ids[0]] = (flow, ids[1:])
    names = list(valves.keys())
    index_to_name: dict[int, str] = dict()
    name_to_index: dict[str, int] = dict()
    valve_list: list[tuple[int, list[str]]] = []
    final_valve_list: list[tuple[int, list[int]]] = []
    for i in range(len(names)):
        index_to_name[i] = names[i]
        name_to_index[names[i]] = i
        valve_list.append(valves[names[i]])
    for i in range(len(valve_list)):
        neighbours = list(map(lambda x: name_to_index[x], valve_list[i][1]))
        final_valve_list.append((valve_list[i][0], neighbours))
    return index_to_name, name_to_index, final_valve_list



if __name__ == "__main__":
    main()


