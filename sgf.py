import networkx as nx

# Graph has to be zero-indexed
def output_graph_sgf(G):
    print("# Simple Graph Format")
    print("# name:", G.name)

    if nx.is_directed(G):
        d = "d"
    else:
        d = "u"
    print(d, G.number_of_nodes(), G.number_of_edges())
    cnt = 0
    for node in G.nodes_iter(data=True):
        node_id = node[0]
        if node_id != cnt:
                raise("non-consecutive node exception")
        #if 'weight' in node[1]:
        cnt += 1
        #print(G.out_edges([node[0]]))
        print(node_id, end="|")
        edges = []
        #edges = [str(edge[1]) for edge in nx.edges_iter(G, [node[0]])]
        for edge in G.edges_iter([node[0]], data='weight'):
            #print('edge', edge)
            src = edge[0]
            dst = edge[1]
            weight = edge[2]
            if src != node_id:
                raise("invalid link")
            if weight != None:
                edges.append(str(dst)+":"+str(weight))
            else:
                edges.append(str(dst))  

        print(",".join(edges))
    if cnt != G.number_of_nodes():
           raise("non-consecutive node exception")
        

G = nx.erdos_renyi_graph(100, 0.1, directed=True)
output_graph_sgf(G)
