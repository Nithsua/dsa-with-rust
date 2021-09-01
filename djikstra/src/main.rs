const N: usize = 9;

fn print_solution(distance: [i32; N]) {
    println!("Vertex\tDistance From Source");
    for (i, v) in distance.iter().enumerate() {
        println!("{}\t\t{}", i, v)
    }
}

fn min_distance_vertex(distance: [i32; N], shortest_path_tree_set: [bool; N]) -> usize {
    let mut min = i32::MAX;
    let mut min_index = 0;

    for (i, v) in distance.iter().enumerate() {
        if *v < min && !shortest_path_tree_set[i] {
            min = *v;
            min_index = i;
        }
    }

    min_index
}

fn djiksta_shortest_path(graph: [[i32; N]; N], inital: usize) {
    let mut distance: [i32; N] = [i32::MAX; N];
    let mut shortest_path_tree_set: [bool; N] = [false; N];

    distance[inital] = 0;

    for _ in 0..(N - 1) {
        let u = min_distance_vertex(distance, shortest_path_tree_set);
        shortest_path_tree_set[u] = true;

        for v in 0..N {
            if !shortest_path_tree_set[v]
                && graph[u][v] != 0
                && distance[u] != i32::MAX
                && distance[u] + graph[u][v] < distance[v]
            {
                distance[v] = distance[u] + graph[u][v];
            }
        }
    }

    print_solution(distance);
}

fn main() {
    let graph: [[i32; N]; N] = [
        [0, 4, 0, 0, 0, 0, 0, 8, 0],
        [4, 0, 8, 0, 0, 0, 0, 11, 0],
        [0, 8, 0, 7, 0, 4, 0, 0, 2],
        [0, 0, 7, 0, 9, 14, 0, 0, 0],
        [0, 0, 0, 9, 0, 10, 0, 0, 0],
        [0, 0, 4, 14, 10, 0, 2, 0, 0],
        [0, 0, 0, 0, 0, 2, 0, 1, 6],
        [8, 11, 0, 0, 0, 0, 1, 0, 7],
        [0, 0, 2, 0, 0, 0, 6, 7, 0],
    ]; // graph from gog

    djiksta_shortest_path(graph, 0);
}
