use std::fs;
use terraform_parser::TerraformParser;

#[test]
fn test_parse() {
    let plan = fs::read_to_string("./tests/plan_create.json").unwrap();
    TerraformParser::parse_plan(&plan).unwrap();
    let plan = fs::read_to_string("./tests/plan_destroy.json").unwrap();
    TerraformParser::parse_plan(&plan).unwrap();
    let plan = fs::read_to_string("./tests/plan_nothing_todo.json").unwrap();
    TerraformParser::parse_plan(&plan).unwrap();
}