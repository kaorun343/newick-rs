extern crate nom;

use crate::tree::{FromNewick, ToNewick};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::space0,
    combinator::{map, opt},
    error::ErrorKind,
    number::complete::double,
    sequence::delimited,
    Err, IResult,
};

/// This function receives a newick format text and returns a phylogenetic tree.
pub fn from_newick<T: FromNewick>(input: &str) -> Result<T, Err<(&str, ErrorKind)>> {
    tree(input).map(|(_, tree)| tree)
}

#[inline]
fn tree<T: FromNewick>(input: &str) -> IResult<&str, T> {
    let (input, tree) = alt((branch, sub_tree))(input)?;
    let (input, _) = tag(";")(input)?;
    Ok((input, tree))
}

fn sub_tree<T: FromNewick>(input: &str) -> IResult<&str, T> {
    alt((internal, leaf))(input)
}

#[inline]
fn leaf<T: FromNewick>(input: &str) -> IResult<&str, T> {
    let (input, name) = name(input)?;
    Ok((input, FromNewick::leaf(name)))
}

#[inline]
fn internal<T: FromNewick>(input: &str) -> IResult<&str, T> {
    let (input, _) = space0(input)?;
    let (input, children) = delimited(tag("("), branch_set, tag(")"))(input)?;
    let (input, name) = name(input)?;
    Ok((input, FromNewick::internal(name, children)))
}

#[inline]
fn branch_set<T: FromNewick>(input: &str) -> IResult<&str, Vec<T>> {
    let (input, _) = space0(input)?;
    alt((
        |input| {
            let (input, branch) = branch(input)?;
            let (input, _) = space0(input)?;
            let (input, _) = tag(",")(input)?;
            let (input, mut branch_set) = branch_set(input)?;

            let mut result = Vec::with_capacity(branch_set.len() + 1);
            result.push(branch);
            result.append(&mut branch_set);

            Ok((input, result))
        },
        map(branch, |branch| vec![branch]),
    ))(input)
}

fn branch<T: FromNewick>(input: &str) -> IResult<&str, T> {
    let (input, sub_tree) = sub_tree::<T>(input)?;
    let (input, length) = length(input)?;
    Ok((input, sub_tree.update_length(length)))
}

#[inline]
fn name(input: &str) -> IResult<&str, String> {
    let (input, _) = space0(input)?;
    let (input, name) = alt((quoted_string, string))(input)?;
    let name = name.to_owned();
    Ok((input, name))
}

#[inline]
fn quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(tag("'"), take_until("'"), tag("'"))(input)
}

#[inline]
fn string(input: &str) -> IResult<&str, &str> {
    take_while(|c| match c {
        ' ' | '(' | ')' | '[' | ']' | '\'' | ':' | ';' | ',' => false,
        _ => true,
    })(input)
}

#[inline]
fn length(input: &str) -> IResult<&str, Option<f64>> {
    opt(|input| {
        let (input, _) = space0(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space0(input)?;
        double(input)
    })(input)
}

/// This function receives a phylogenetic tree and returns a newick text.
pub fn to_newick<T: ToNewick>(tree: &T) -> String {
    format_tree(tree)
}

#[inline]
fn format_tree<T: ToNewick>(tree: &T) -> String {
    format!("{};", format_sub_tree(tree))
}

fn format_sub_tree<T: ToNewick>(tree: &T) -> String {
    let children = {
        let children = tree.get_children();
        if children.is_empty() {
            String::new()
        } else {
            let branch_set = children
                .into_iter()
                .map(format_sub_tree)
                .collect::<Vec<_>>()
                .join(",");
            format!("({})", branch_set)
        }
    };

    let name = tree.get_name();

    let length = tree
        .get_length()
        .map_or(String::new(), |length| format!(":{}", length));

    format!("{}{}{}", children, name, length)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::simple_tree::SimpleTree;

    #[test]
    fn test_from_newick() {
        type T = SimpleTree;
        assert!(from_newick::<T>("(,,(,));").is_ok());
        assert!(from_newick::<T>("(A,B,(C,D));").is_ok());
        assert!(from_newick::<T>("(A,B,(C,D)E)F;").is_ok());
        assert!(from_newick::<T>("(:0.1,:0.2,(:0.3,:0.4):0.5);").is_ok());
        assert!(from_newick::<T>("(:0.1,:0.2,(:0.3,:0.4):0.5):0.0;").is_ok());
        assert!(from_newick::<T>("(A:0.1,B:0.2,(C:0.3,D:0.4):0.5);").is_ok());
        assert!(from_newick::<T>("(A:0.1,B:0.2,(C:0.3,D:0.4)E:0.5)F;").is_ok());
        assert!(from_newick::<T>("((B:0.2,(C:0.3,D:0.4)E:0.5)A:0.1)F;  ").is_ok());
        assert!(from_newick::<T>("(A A,B,(C,D));").is_err());
        assert!(from_newick::<T>("(,,(,)) ;").is_ok());
    }

    #[test]
    fn test_leaf() {
        assert_eq!(
            leaf("A:0.3"),
            Ok((":0.3", SimpleTree::new("A".to_owned(), None, Vec::new())))
        );
    }

    #[test]
    fn test_internal() {
        assert_eq!(
            internal("(A:0.1,B:0.5)1000:0.3"),
            Ok((
                ":0.3",
                SimpleTree::new(
                    "1000".to_owned(),
                    None,
                    vec![
                        SimpleTree::new("A".to_owned(), Some(0.1), Vec::new()),
                        SimpleTree::new("B".to_owned(), Some(0.5), Vec::new()),
                    ]
                )
            ))
        );
        assert_eq!(
            internal("( A : 0.1 , B : 0.5) 1000 :0.3"),
            Ok((
                " :0.3",
                SimpleTree::new(
                    "1000".to_owned(),
                    None,
                    vec![
                        SimpleTree::new("A".to_owned(), Some(0.1), Vec::new()),
                        SimpleTree::new("B".to_owned(), Some(0.5), Vec::new()),
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_branch_set() {
        assert_eq!(
            branch_set(",,"),
            Ok((
                "",
                vec![
                    SimpleTree::new("".to_owned(), None, Vec::new()),
                    SimpleTree::new("".to_owned(), None, Vec::new()),
                    SimpleTree::new("".to_owned(), None, Vec::new())
                ]
            ))
        );
        assert_eq!(
            branch_set(", ,"),
            Ok((
                "",
                vec![
                    SimpleTree::new("".to_owned(), None, Vec::new()),
                    SimpleTree::new("".to_owned(), None, Vec::new()),
                    SimpleTree::new("".to_owned(), None, Vec::new())
                ]
            ))
        );
    }

    #[test]
    fn test_branch() {
        assert_eq!(
            branch("A:0.1"),
            Ok(("", SimpleTree::new("A".to_owned(), Some(0.1), Vec::new())))
        );

        assert_eq!(
            branch(" A : 0.1"),
            Ok(("", SimpleTree::new("A".to_owned(), Some(0.1), Vec::new())))
        );
    }

    #[test]
    fn test_name() {
        assert_eq!(name(""), Ok(("", "".to_owned())));
        assert_eq!(name(":50"), Ok((":50", "".to_owned())));
        assert_eq!(name("A:50"), Ok((":50", "A".to_owned())));
        assert_eq!(name("A B"), Ok((" B", "A".to_owned())));
        assert_eq!(name("'A B'"), Ok(("", "A B".to_owned())));
    }

    #[test]
    fn test_length() {
        assert_eq!(length(""), Ok(("", None)));
        assert_eq!(length(",D"), Ok((",D", None)));
        assert_eq!(length(":0.1,D"), Ok((",D", Some(0.1))));
        assert_eq!(length(" :0.1"), Ok(("", Some(0.1))));
        assert_eq!(length(": 0.1"), Ok(("", Some(0.1))));
    }

    #[test]
    fn test_to_newick() {
        type T = SimpleTree;
        vec![
            "(,,(,));",
            "(A,B,(C,D));",
            "(A,B,(C,D)E)F;",
            "(:0.1,:0.2,(:0.3,:0.4):0.5);",
            "(:0.1,:0.2,(:0.3,:0.4):0.5):0.6;",
            "(A:0.1,B:0.2,(C:0.3,D:0.4):0.5);",
            "(A:0.1,B:0.2,(C:0.3,D:0.4)E:0.5)F;",
        ]
        .into_iter()
        .for_each(|newick| {
            assert_eq!(
                to_newick(&from_newick::<T>(newick).unwrap()),
                newick.to_owned()
            );
        });
    }
}
