extern crate aoc;

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Mult,
}

#[derive(Debug)]
enum Node {
    Branch(Branch),
    Leaf(i64)
}

#[derive(Debug)]
struct Branch {
    operator: Operator,
    left: Box<Node>,
    right: Box<Node>,
}

fn tokenize(input: &str) -> Vec<Vec<&str>> {
    let mut tokenized_exprs = vec![];
    for line in input.lines() {
        let mut expr_tokens = vec![];
        let mut last = 0;
        for (index, matched) in line.match_indices(| c: char | c == ' ' || c == '(' || c == ')') {
            if last != index {
                expr_tokens.push(&line[last..index]);
            }
            if matched != " " {
                expr_tokens.push(matched);
            }
            last = index + matched.len();
        }
        if last < line.len() {
            expr_tokens.push(&line[last..]);
        }
        tokenized_exprs.push(expr_tokens);
    }

    tokenized_exprs
}

fn get_node(mut expr_tokens: Vec<&str>) -> Node {
    let mut highest_op: Option<(Operator, usize)> = None;

    let mut in_parens = 0;
    for (i, &tok) in expr_tokens.iter().enumerate() {
        match tok {
            "(" => in_parens += 1,
            ")" => in_parens -= 1,
            _ => {
                if in_parens == 0 {
                    match tok {
                        "+" => {
                            highest_op = Some((Operator::Add, i));
                        },
                        "*" => {
                            highest_op = Some((Operator::Mult, i));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    match highest_op {
        Some((op, ind)) => {
            Node::Branch(Branch {
                operator: op,
                left: Box::new(get_node((&expr_tokens[..ind]).to_vec().clone())),
                right: Box::new(get_node((&expr_tokens[ind + 1..]).to_vec().clone()))
            })
        },
        None => {
            if expr_tokens[0] == "(" && expr_tokens[expr_tokens.len() - 1] == ")" {
                expr_tokens.remove(0);
                expr_tokens.pop();
                get_node(expr_tokens)
            } else {
                if expr_tokens.len() == 1 {
                    Node::Leaf(expr_tokens[0].parse::<i64>().unwrap())
                } else {
                    println!("DUMP: {:?}", expr_tokens);
                    panic!("BUhhhhhhh how did we get here hmm")
                }
            }
        }
    }
}

fn eval_node(node: Node) -> i64 {
    match node {
        Node::Branch(branch) => {
            match branch.operator {
                Operator::Add => eval_node(*branch.left) + eval_node(*branch.right),
                Operator::Mult => eval_node(*branch.left) * eval_node(*branch.right)
            }
        },
        Node::Leaf(val) => val
    }
}

fn main() {
    let input = aoc::input_to_str("18");
    let tokenized_exprs = tokenize(&input);

    println!("{:?}", get_node(tokenized_exprs[0].clone()));

    let mut sum: i64 = 0;
    for tokens in tokenized_exprs {
        let n = eval_node(get_node(tokens));
        println!("{}", n);
        sum += n;
    }

    println!("Result: {}", sum);
}
