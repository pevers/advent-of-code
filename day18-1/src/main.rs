use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
struct Node {
    value: i8,
    left: Option<NodeType>,
    right: Option<NodeType>,
}

type NodeType = Rc<RefCell<Node>>;

fn digit(i: &str) -> IResult<&str, Node> {
    map(digit1, |s: &str| Node {
        value: s.parse().unwrap(),
        left: None,
        right: None,
    })(i)
}

fn expr(i: &str) -> IResult<&str, (Node, Node)> {
    separated_pair(alt((parens, digit)), tag(","), alt((parens, digit)))(i)
}

fn parens(i: &str) -> IResult<&str, Node> {
    map(delimited(tag("["), expr, tag("]")), |(left, right)| Node {
        value: -1,
        left: Some(Rc::new(RefCell::new(left))),
        right: Some(Rc::new(RefCell::new(right))),
    })(i)
}

fn parse(i: &str) -> Node {x
    let (_, node) = parens(i).unwrap();
    node
}

fn find_leftmost_nested(node: &NodeType, times: u32) -> Option<NodeType> {
    if times == 1 {
        return Some(node.clone());
    }
    let node = node.as_ref().borrow();
    if let Some(left) = &node.left {
        return find_leftmost_nested(&left.clone(), times - 1);
    }
    if let Some(right) = &node.right {
        return find_leftmost_nested(&right.clone(), times - 1);
    }
    None
}

fn add(left: Node, right: Node) -> Node {
    let sum = Node {
        value: -1,
        left: Some(Rc::new(RefCell::new(left))),
        right: Some(Rc::new(RefCell::new(right))),
    };
    sum
}

fn main() {
    include_str!("../input").split("\n").fold(
        Node {
            value: -1,
            left: None,
            right: None,
        },
        |accum, curr| {
            let snail = parse(curr);
            add(snail, accum)
        },
    );
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{find_leftmost_nested, parens, parse};

    #[test]
    fn test_parse() {
        let input = "[[1,2],[2,3]]";
        println!("{:?}", parens(input).unwrap());
    }

    #[test]
    fn test_leftmost_pair() {
        let input = "[[[[[9,8],1],2],3],4]";
        let node = parse(&input);
        let link = Rc::new(RefCell::new(node));
        let nested_pair = find_leftmost_nested(&link, 5).unwrap();
        println!("{:?}", &nested_pair);
    }
}
