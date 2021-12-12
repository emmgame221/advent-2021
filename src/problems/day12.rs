use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque};

pub fn print_solution() {
    let input = File::open("./inputs/test_input12.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let mut graph = Graph::new();
    for line in lines {
        let line = line.unwrap();
        graph.parse_line(&line);
    }
    //println!("{:?}", graph);
    let p1 = graph.count_paths();
    println!("Day 12 Part 1: {}", p1);
}

#[derive(Debug)]
enum NodeType {
    Start,
    Big,
    Small,
    End,
}

impl NodeType {
    fn from_str(s: &str) -> Self {
        match s {
            "start" => NodeType::Start,
            "end" => NodeType::End,
            s => {
                if s.chars().all(|c| c.is_ascii_uppercase()) {
                    NodeType::Big
                } else {
                    NodeType::Small
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    ntype: NodeType,
    edges: Vec<String>,
}

impl Node {
    fn new(ntype: &str, name: String) -> Self {
        let ntype = NodeType::from_str(ntype);
        Self {
            name,
            ntype,
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, v: &str) {
        self.edges.push(v.to_owned());
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    fn parse_line(&mut self, line: &str) {
        let parts: Vec<&str> = line.split('-').collect();
        let u = parts[0];
        let v = parts[1];
        self.insert_edge(u, v);
    }

    fn insert_edge(&mut self, u: &str, v: &str) {
        if let Some(u_node) = self.nodes.iter_mut().find(|n| n.name == u) {
            u_node.add_edge(v);
        } else {
            let mut u_node = Node::new(u, u.to_owned());
            u_node.add_edge(v);
            self.nodes.push(u_node);
        }
        if let Some(v_node) = self.nodes.iter_mut().find(|n| n.name == v) {
            v_node.add_edge(u);
        } else {
            let mut v_node = Node::new(v, v.to_owned());
            v_node.add_edge(u);
            self.nodes.push(v_node);
        }
    }

    fn count_paths(&self) -> usize {
        let mut total = 0;
        let mut queue: VecDeque<&Node> = VecDeque::new();
        let mut small_visited: Vec<&String> = Vec::new();
        let start = self.nodes.iter().find(|n| n.name == "start").unwrap();
        queue.push_back(start);
        small_visited.push(&start.name);
        while !queue.is_empty() {
            println!("queue:{:?}", queue.iter().map(|n| &n.name).collect::<Vec<&String>>());
            let u = queue.pop_front().unwrap();
            println!("u:{}", u.name);
            for v in u.edges.iter() {
                let v = self.nodes.iter().find(|n| n.name == *v).unwrap();
                match v.ntype {
                    NodeType::Small | NodeType::Start => {
                        if !small_visited.contains(&&v.name) {
                            queue.push_back(v);
                            small_visited.push(&v.name);
                            println!("v:{}", v.name);
                        }
                    }
                    NodeType::End => {
                        println!("v:{}", v.name);
                        total += 1;
                    }
                    _ => {
                        println!("v:{}", v.name);
                        queue.push_back(v);
                    }
                }
            }
        }
        total
    }
}