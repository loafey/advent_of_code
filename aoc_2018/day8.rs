use utils::load_string;

#[derive(Debug)]
struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>,
}
fn parse_node<'l>(data: &mut impl Iterator<Item = &'l mut usize>) -> Node {
    let child_amount = *data.next().unwrap();
    let meta_data_amount = *data.next().unwrap();
    let children = (0..child_amount).map(|_| parse_node(data)).collect();
    let metadata = (0..meta_data_amount)
        .map(|_| *data.next().unwrap())
        .collect();

    Node { metadata, children }
}

pub fn part1() -> usize {
    fn count_metadata(tree: Node) -> usize {
        tree.metadata.into_iter().sum::<usize>()
            + tree.children.into_iter().map(count_metadata).sum::<usize>()
    }

    let mut nums = load_string("inputs/2018/day8.input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    count_metadata(parse_node(&mut nums.iter_mut()))
}

pub fn part2() -> usize {
    fn worth(tree: &Node) -> usize {
        if tree.children.is_empty() {
            tree.metadata.iter().sum()
        } else {
            let mut value = 0;
            for index in &tree.metadata {
                let index = index - 1;
                if index < tree.children.len() {
                    value += worth(&tree.children[index]);
                }
            }
            value
        }
    }

    let mut nums = load_string("inputs/2018/day8.input")
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    worth(&parse_node(&mut nums.iter_mut()))
}
