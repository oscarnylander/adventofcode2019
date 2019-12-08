use ego_tree::{NodeMut, NodeRef, Tree};

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

fn common_ancestor<'a>(
    first: &NodeRef<'a, String>,
    second: &NodeRef<'a, String>,
) -> NodeRef<'a, String> {
    let first_ancestors = first.ancestors().collect::<Vec<_>>();
    let second_ancestors = second.ancestors().collect::<Vec<_>>();

    let mut common_ancestors = first_ancestors
        .iter()
        .filter(|fa| second_ancestors.iter().any(|sa| fa.id() == sa.id()))
        .collect::<Vec<_>>();

    *common_ancestors.remove(0)
}

fn distance_to_child(
    parent: &NodeRef<String>,
    child: &NodeRef<String>,
    current_depth: usize,
) -> Option<usize> {
    let mut current_depth = current_depth;
    current_depth += 1;
    for n in parent.children() {
        if n.id() == child.id() {
            return Some(current_depth);
        }
        if let Some(found) = distance_to_child(&n, child, current_depth) {
            return Some(found);
        }
    }
    None
}

#[aoc(day6, part2)]
fn solve_2(input: &[Edge]) -> usize {
    let trees = to_trees(&input);
    let tree = &trees[0];

    let you = tree.nodes().find(|n| n.value() == "YOU").unwrap();

    let san = tree.nodes().find(|n| n.value() == "SAN").unwrap();

    let common = common_ancestor(&you, &san);

    let distance_you = distance_to_child(&common, &you, 0).unwrap();
    let distance_san = distance_to_child(&common, &san, 0).unwrap();

    distance_you + distance_san - 2
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

    #[allow(dead_code)]
    const EXAMPLE_2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

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

    #[test]
    fn common_ancestor_works_on_part_2_example_1() {
        let input = generate_input(EXAMPLE_2);
        let trees = to_trees(&input);
        let tree = &trees[0];

        let you = tree.nodes().find(|n| n.value() == "YOU").unwrap();

        let san = tree.nodes().find(|n| n.value() == "SAN").unwrap();

        assert_eq!(common_ancestor(&you, &san).value(), "D");
    }

    #[test]
    fn distance_to_child_works_on_part_2_example_1() {
        let input = generate_input(EXAMPLE_2);
        let trees = to_trees(&input);
        let tree = &trees[0];

        let you = tree.nodes().find(|n| n.value() == "YOU").unwrap();

        let san = tree.nodes().find(|n| n.value() == "SAN").unwrap();

        let d = common_ancestor(&you, &san);

        assert_eq!(distance_to_child(&d, &you, 0), Some(4));
        assert_eq!(distance_to_child(&d, &san, 0), Some(2));
    }
}
