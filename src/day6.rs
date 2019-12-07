use ego_tree::{NodeMut, Tree};

#[derive(Debug)]
pub struct Edge {
    parent: String,
    child: String,
}

#[aoc_generator(day6)]
pub fn generate_input(input: &str) -> Vec<Edge> {
    input
        .lines()
        .map(|i| {
            let mut parts = i.split(')');
            Edge {
                parent: parts.next().unwrap().to_owned(),
                child: parts.next().unwrap().to_owned(),
            }
        })
        .collect::<Vec<_>>()
}

fn find_roots(edges: &[Edge]) -> Vec<String> {
    let parents = edges.iter().map(|e| &e.parent).collect::<Vec<_>>();

    parents
        .iter()
        .filter(|pr| {
            (parents.iter().filter(|i| i == pr).count() == 1)
                && edges.iter().filter(|e| e.child == ***pr).count() == 0
        })
        .map(|r| r.to_owned().to_owned())
        .collect::<Vec<_>>()
}

fn add_children(node: &mut NodeMut<String>, edges: &[Edge]) {
    let value = node.value().to_owned();
    let children = edges.iter().filter(|e| e.parent == value);
    children.for_each(|e| {
        let mut new_node = node.append(e.child.to_owned());
        add_children(&mut new_node, &edges);
    });
}

fn to_trees(edges: &[Edge]) -> Vec<Tree<String>> {
    let roots = find_roots(&edges);

    roots
        .iter()
        .map(|root| {
            let mut tree = Tree::new(root.to_owned());
            let mut root = tree.root_mut();
            add_children(&mut root, &edges);
            tree
        })
        .collect::<Vec<_>>()
}

fn summed_orbits(trees: Vec<Tree<String>>) -> usize {
    trees
        .iter()
        .map::<usize, _>(|tree| {
            tree.root()
                .descendants()
                .map(|n| n.ancestors().count())
                .sum()
        })
        .sum()
}

#[aoc(day6, part1)]
fn solve_1(input: &[Edge]) -> usize {
    summed_orbits(to_trees(input))
}

mod tests {
    #[allow(dead_code)]
    const EXAMPLE_1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn find_root_works_on_example_1() {
        let input = generate_input(EXAMPLE_1);

        assert_eq!(find_roots(&input), vec!["COM"]);
    }

    #[test]
    fn to_tree_works_on_example_1() {
        use ego_tree::tree;

        let input = generate_input(EXAMPLE_1);

        let desired_output = tree!(
            "COM".to_string() => {
                "B".to_string() => {
                    "C".to_string() => {
                        "D".to_string() => {
                            "E".to_string() => {
                                "F".to_string(),
                                "J".to_string() => {
                                    "K".to_string() => {
                                        "L".to_string()
                                    }
                                }
                            },
                            "I".to_string()
                        },
                    },
                    "G".to_string() => {
                        "H".to_string()
                    }
                }
            }
        );
        let actual_output = to_trees(&input);

        assert_eq!(actual_output[0], desired_output);
    }

    #[test]
    fn it_solves_part_1_example_1() {
        let input = generate_input(EXAMPLE_1);
        let trees = to_trees(&input);

        assert_eq!(summed_orbits(trees), 42);
    }
}
