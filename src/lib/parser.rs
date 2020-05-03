use crate::lib::data::DataPre;
use crate::pest::{
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(Parser)]
#[grammar = "lib/lisp.pest"]
struct LispParser;

fn pairs_to_data(pair: Pair<Rule>) -> DataPre {
    match pair.as_rule() {
        Rule::expr => {
            let mut inner = pair.into_inner();
            let inner_str = inner.as_str();
            match inner.clone().next().unwrap().as_rule() {
                // unwrap() is safe because expr's only contain one inner element
                Rule::list => DataPre::List(
                    inner
                        .next()
                        .unwrap()
                        .into_inner()
                        .map(pairs_to_data)
                        .collect(),
                ),
                Rule::int => DataPre::Int(inner_str.to_string()),
                Rule::float => DataPre::Float(inner_str.to_string()),
                Rule::string => DataPre::Str(parse_string(inner)),
                Rule::symbol => DataPre::Symbol(inner_str.to_string()),
                any_other => unreachable!("inside expr: {:?}", any_other),
            }
        }
        Rule::EOI => DataPre::Nil, // The end of the file is simply ignored
        any_other => unreachable!("{:?}", any_other),
    }
}

fn parse_string(string_data: Pairs<Rule>) -> String {
    let chars = string_data
        .clone() // string
        .next()
        .unwrap()
        .into_inner() // string_inner
        .next()
        .unwrap()
        .into_inner(); // char*
    let mut final_string = String::new();

    for ch in chars {
        let ch_data = ch.into_inner().next().unwrap();
        let ch_rule = ch_data.clone().as_rule();
        let ch_str = ch_data.clone().as_str();

        match ch_rule {
            Rule::char_normal => final_string.push_str(ch_str),
            Rule::char_escape_code => final_string.push(match ch_str {
                "\\n" => '\n',
                "\\t" => '\t',
                "\\\"" => '\"',
                "\\\\" => '\\',
                _ => unreachable!("this escape code should not be here: '{}'", ch_str),
            }),
            // I'm struggling to implement this one:
            // Rule::char_unicode_hex => {
            //     for byte_step in 0..1 {
            //         final_string.push(
            //             std::char::from_u32(
            //                 u32::from_str_radix(&ch_str[(2 + byte_step)..(4 + byte_step)], 16)
            //                     .unwrap(),
            //             )
            //             .unwrap(),
            //         )
            //     }
            // }
            _ => unreachable!("{:?}", ch_rule),
        }
    }

    final_string
}

pub fn parse_program(program: &str) -> Result<Vec<DataPre>, pest::error::Error<Rule>> {
    match LispParser::parse(Rule::program, program) {
        Ok(mut program) => {
            let mut data: Vec<DataPre> = program
                .next()
                .unwrap()
                .into_inner()
                .map(pairs_to_data)
                .collect();
            data.pop(); // remove DataPre::Nil resulted from EOI
            Ok(data)
        }
        Err(e) => Err(e),
    }
}
