#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "../tests/timeout.pest"]
struct TimingOutParser;

#[test]
fn fn_call() {
    parses_to! {
        parser: TimingOutParser,
        input: "_(p=__(p=[_(p=__(p=[_(p=[_(p=[_1(p=[_(p=[_1(p=[_(p=[_1(p=[_(p=[_1(p=[_(p=[_1",
        rule: Rule::fn_call,
        tokens: [

        ]
    };
}
