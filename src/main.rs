use nom::character::complete::{alphanumeric1, space1};
use nom::combinator::opt;
use nom::Parser as NomParser;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space0},
    combinator::map,
    multi::many1,
    IResult,
};

type Parser<'a, T> = IResult<&'a str, T>;

#[derive(Debug, PartialEq)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, PartialEq)]
struct NodeId(pub String);

#[derive(Debug, PartialEq)]
struct FilePath(pub String);

#[derive(Debug, PartialEq)]
struct Node {
    node_id: NodeId,
    file_path: FilePath,
}

#[derive(Debug, PartialEq)]
struct Edge(pub NodeId, pub NodeId);

#[derive(Debug, PartialEq)]
pub enum Lexer {
    N(Node),
    E(Edge),
}

fn parse_edges(i: &str) -> Parser<Edge> {
    let (i, _) = tag("(")(i)?;
    let (i, a) = alphanumeric1(i)?;
    let (i, _) = space0(i)?;
    let (i, _) = tag("->")(i)?;
    let (i, _) = space0(i)?;
    let (i, b) = alphanumeric1(i)?;
    let (i, _) = tag(")")(i)?;

    Ok((i, Edge(NodeId(a.to_string()), NodeId(b.to_string()))))
}

fn parse_file_path(i: &str) -> Parser<FilePath> {
    let (i, _) = tag("'")(i)?;
    let (i, fpath) = alphanumeric1(i)?;
    let (i, fpath_extension) = parse_file_extension(i)?;
    let (i, _) = tag("'")(i)?;

    Ok((i, FilePath(fpath.to_owned() + &fpath_extension)))
}

fn parse_file_extension(i: &str) -> Parser<String> {
    let (i, maybe_dot) = opt(tag(".")).parse(i)?;
    let (i, maybe_extension) = opt(alphanumeric1).parse(i)?;

    Ok((
        i,
        maybe_dot.unwrap_or("").to_owned() + maybe_extension.unwrap_or(""),
    ))
}

fn parse_node(i: &str) -> Parser<Node> {
    let (i, _) = tag("(")(i)?;
    let (i, _) = tag(".")(i)?;
    let (i, n_id) = alphanumeric1(i)?;
    let (i, _) = space1(i)?;
    let (i, fpath) = parse_file_path(i)?;
    let (i, _) = tag(")")(i)?;

    Ok((
        i,
        Node {
            node_id: NodeId(n_id.to_string()),
            file_path: fpath,
        },
    ))
}

fn parse_item(i: &str) -> Parser<Lexer> {
    let (i, lst) = alt((map(parse_node, Lexer::N), map(parse_edges, Lexer::E))).parse(i)?;

    Ok((i, lst))
    // Ok((
    //     i,
    //     Graph {
    //         nodes: vec![],
    //         edges: vec![],
    //     },
    // ))
}

fn parse_items(i: &str) -> Parser<Vec<Lexer>> {
    many1(parse_item).parse(i)
}

fn main() {
    let file_path = "data/example1.drs";
    let contents = std::fs::read_to_string(file_path).expect("no file");
    let contents_without_newlines = contents.lines().collect::<Vec<_>>().join("");
    let r = parse_items(&contents_without_newlines);
    println!("{:?}", r);

    println!("Parse complete!");
}
