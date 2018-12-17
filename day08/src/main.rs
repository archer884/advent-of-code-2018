#[derive(Debug)]
struct Node<'a> {
    children: Vec<Node<'a>>,
    metadata: &'a [i32],
    len: usize,
}

impl Node<'_> {
    fn checksum(&self) -> i32 {
        self.metadata.iter().cloned().sum::<i32>()
            + self
                .children
                .iter()
                .map(|x| x.checksum())
                .sum::<i32>()
    }

    fn len(&self) -> usize {
        self.len
    }
}

struct Header {
    children: usize,
    metadata: usize,
}

impl Header {
    fn extract(license: &[i32]) -> Result<Header, NodeError> {
        Ok(Header {
            children: *license.get(0).ok_or(NodeError::Header)? as usize,
            metadata: *license.get(1).ok_or(NodeError::Header)? as usize,
        })
    }
}

#[derive(Debug)]
enum NodeError {
    Header,
    Body,
}

fn main() -> Result<(), NodeError> {
    let license = license_values();
    let tree = build_tree(&license)?;

    println!("{}", tree.checksum());

    Ok(())
}

fn build_tree(license: &[i32]) -> Result<Node, NodeError> {
    // Offset is initialized to the width of the header.
    let mut offset = 2;
    let mut children = Vec::new();

    let header = Header::extract(license)?;

    for _ in 0..header.children {
        let child = build_tree(&license[offset..])?;
        offset += child.len();
        children.push(child);
    }

    // Hopefully this just never happens...
    let metadata = &license[offset..(offset + (header.metadata as usize))];
    if metadata.len() != header.metadata {
        return Err(NodeError::Body);
    }

    Ok(Node {
        children,
        metadata,
        len: offset + header.metadata,
    })
}

fn license_values() -> Vec<i32> {
    static INPUT: &str = include_str!("../input.txt");

    fn parse_int(s: &str) -> Option<i32> {
        s.parse().ok()
    }

    INPUT.split_whitespace().filter_map(parse_int).collect()
}
