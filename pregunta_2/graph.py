from typing import Dict, List



def dfs(graph : Dict[str, List[str]], start_node : str, length : int = 0) -> int:
    maxi = -1

    for node in graph[start_node]:
        maxi = max(dfs(graph, node, length + 1), maxi)

    return max(maxi, length)



def create_graph(list : List[str]) -> Dict[str, List[str]]:
    graph = {}
    for s in list:
        (a, rel, b) = s.split()
        if rel == "<":
            start = "g_"+b; end = "f_"+a
        elif rel == ">":
            start = "f_" + a; end = "g_"+b

        if not (l := graph.get(start)):
            l = graph[start] = []
        l.append(end)

        if not graph.get(end): graph[end] = []

    return graph

graph = [
    "try < try",
    "try < ;",
    "try < instr",
    "try > $",
    "catch < try",
    "catch > catch",
    "catch < ;",
    "catch < instr",
    "catch > $",
    "finally < try",
    "finally > catch",
    "finally > finally",
    "finally > ;",
    "finally < instr",
    "finally > $",
    "; < try",
    "; > catch",
    "; > finally",
    "; > ;",
    "; < instr",
    "; > $",
    "instr > try",
    "instr > catch",
    "instr > finally",
    "instr > ;",
    "instr > $",
    "$ < try",
    "$ < catch",
    "$ < finally",
    "$ < ;",
    "$ < instr",
]

graph = create_graph(graph)

results = {}
for key in graph.keys():
    results[key] = dfs(graph, key)

# Printing graph
for (start, l) in graph.items():
    for end in l:
        print(f"{start} {end}")

# Printing values for f and g functions
keywords = ["try", "catch", "finally", ";", "instr", "$"]
for word in keywords:
    print(f"{word}: f = {results['f_' + word]}, g = {results['g_' + word]}")