use pest::Parser;

#[derive(Parser)]
#[grammar = "bab.pest"]
pub struct BabParser;

#[test]
fn test_lexer() {
    let pairs = BabParser::parse(Rule::grammar, "Var test :").unwrap();
    let tokens: Vec<_> = pairs.flatten().collect();
    for token in tokens {
        println!("{:?}", token);
    }
    assert_eq!(true, true);
 }
