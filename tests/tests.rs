use std::fs;
use terraform_parser::TerraformParser;

#[test]
fn test_parse() {
    let plan = fs::read_to_string("./tests/plan.json").unwrap();
    TerraformParser::parse_plan(&plan).unwrap();
    let plan = fs::read_to_string("./tests/plan2.json").unwrap();
    TerraformParser::parse_plan(&plan).unwrap();
}