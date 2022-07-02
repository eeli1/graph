use graph::Graph;

#[derive(Clone, PartialEq, Debug)]

struct Coordinates {
    lan: f32,
    lon: f32,
}

impl Coordinates {
    fn new(lan: f32, lon: f32) -> Self {
        Self { lan, lon }
    }
}

fn load(lines: Vec<&str>, is_bi: bool) -> Graph<Coordinates, usize> {
    let n = lines[0].parse().unwrap();
    let m = lines[1].parse().unwrap();

    let mut graph = Graph::new();

    for i in 0..n {
        let split: Vec<&str> = lines[i + 2].split(" ").collect();
        graph
            .add_node(Coordinates::new(
                split[0].parse().unwrap(),
                split[1].parse().unwrap(),
            ))
            .unwrap();
    }

    for i in 0..m {
        let split: Vec<&str> = lines[i + n + 2].split(" ").collect();
        graph
            .add_edge(
                split[0].parse().unwrap(),
                split[1].parse().unwrap(),
                split[2].parse().unwrap(),
            )
            .unwrap();
        if is_bi {
            graph
                .add_edge(
                    split[1].parse().unwrap(),
                    split[0].parse().unwrap(),
                    split[2].parse().unwrap(),
                )
                .unwrap();
        }
    }

    graph
}

#[test]
fn mst_small() {
    let lines = include_str!("graph_small.txt").split("\n").collect();
    let graph = load(lines, true);

    let mst = graph::algo::find_mst(graph, |x| x.clone());
    let lines: Vec<&str> = include_str!("mst_small.txt").split("\n").collect();
    let out: Vec<(usize, usize, usize)> = lines
        .iter()
        .map(|&s| -> (usize, usize, usize) {
            let sp: Vec<&str> = s.split(" ").collect();
            (
                sp[0].parse().unwrap(),
                sp[1].parse().unwrap(),
                sp[2].parse().unwrap(),
            )
        })
        .collect();

    assert_eq!(out, mst);
}

#[test]
fn mst_big() {
    let lines = include_str!("graph_big.txt").split("\n").collect();
    let graph = load(lines, true);

    let mst = graph::algo::find_mst(graph, |x| x.clone());
    let lines: Vec<&str> = include_str!("mst_big.txt").split("\n").collect();
    let out: Vec<(usize, usize, usize)> = lines
        .iter()
        .map(|&s| -> (usize, usize, usize) {
            let sp: Vec<&str> = s.split(" ").collect();
            (
                sp[0].parse().unwrap(),
                sp[1].parse().unwrap(),
                sp[2].parse().unwrap(),
            )
        })
        .collect();

    assert_eq!(out, mst);
}
