use std::char;

pub mod test;

pub fn find_digit(
    chars: &mut dyn Iterator<Item = char>,
    nodes: Vec<(char, Vec<usize>, Option<usize>)>,
) -> Option<usize> {
    let mut curr_node = nodes[0].clone();
    for char in chars {
        if let Some(digit) = char.to_digit(10) {
            return Some(digit as usize);
        }
        let next_node = curr_node.1.iter().find(|node| nodes[**node].0 == char);
        if let Some(next_node) = next_node {
            curr_node = nodes[*next_node].clone();
        } else {
            curr_node = nodes[0].clone();
            let next_node = curr_node.1.iter().find(|node| nodes[**node].0 == char);
            if let Some(next_node) = next_node {
                curr_node = nodes[*next_node].clone();
            } else {
                curr_node = nodes[0].clone();
            }
        }
        if let Some(digit) = curr_node.2 {
            return Some(digit);
        }
    }
    None
}

pub fn recover_calibration(dirty_data: &str) -> usize {
    let nodes: Vec<_> = vec![
        ('?', vec![1, 4, 11, 18, 25, 30], None),
        ('o', vec![2], None),
        ('n', vec![3, 31], None),
        ('e', vec![], Some(1)),
        ('t', vec![5, 7], None),
        ('w', vec![6], None),
        ('o', vec![], Some(2)),
        ('h', vec![8], None),
        ('r', vec![9], None),
        ('e', vec![10, 27], None),
        ('e', vec![], Some(3)),
        ('f', vec![12, 15], None),
        ('o', vec![13, 2], None),
        ('u', vec![14], None),
        ('r', vec![], Some(4)),
        ('i', vec![16], None),
        ('v', vec![17], None),
        ('e', vec![], Some(5)),
        ('s', vec![19, 21], None),
        ('i', vec![20], None),
        ('x', vec![], Some(6)),
        ('e', vec![22], None),
        ('v', vec![23], None),
        ('e', vec![24, 26], None),
        ('n', vec![], Some(7)),
        ('e', vec![26], None),
        ('i', vec![27], None),
        ('g', vec![28], None),
        ('h', vec![29], None),
        ('t', vec![], Some(8)),
        ('n', vec![31], None),
        ('i', vec![32], None),
        ('n', vec![33, 31], None),
        ('e', vec![], Some(9)),
    ];
    let nodes_reversed: Vec<_> = vec![
        ('?', vec![1, 13, 16, 20, 23, 28], None),
        ('e', vec![2, 5, 8], None),
        ('n', vec![3, 12], None),
        ('i', vec![4], None),
        ('n', vec![], Some(9)),
        ('v', vec![6], None),
        ('i', vec![7], None),
        ('f', vec![], Some(5)),
        ('e', vec![33, 8], None),
        ('h', vec![10], None),
        ('t', vec![], Some(3)),
        ('.', vec![], None), // Useless node , too lazy to rewrite the whole tree
        ('o', vec![], Some(1)),
        ('o', vec![14], None),
        ('w', vec![15], None),
        ('t', vec![], Some(2)),
        ('r', vec![17], None),
        ('u', vec![18], None),
        ('o', vec![19], None),
        ('f', vec![], Some(4)),
        ('x', vec![21], None),
        ('i', vec![22], None),
        ('s', vec![], Some(6)),
        ('n', vec![24], None),
        ('e', vec![25], None),
        ('v', vec![26], None),
        ('e', vec![27], None),
        ('s', vec![], Some(7)),
        ('t', vec![29], None),
        ('h', vec![30], None),
        ('g', vec![31], None),
        ('i', vec![32], None),
        ('e', vec![], Some(8)),
        ('r',vec![9],None)
    ];

    dirty_data.split("\n").fold(0, |acc, line| {
        let chars = line.chars().into_iter();
        let first = find_digit(&mut chars.clone(), nodes.clone()).unwrap();
        let last = find_digit(&mut chars.rev(), nodes_reversed.clone()).unwrap();

        acc + first * 10 + last
    })
}
