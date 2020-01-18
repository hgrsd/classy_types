use std::fmt::Write;

pub fn classify(file_contents: &str) -> String {
    let mut classified_contents = String::new();
    for line in file_contents.lines() {
        let mut classified_line = line.to_string();
        if line.contains("export interface") {
            classified_line = generate_class_for_interface(line);
        } else if line.contains("export declare type") && !line.contains("|") {
            classified_line = generate_class_for_type(line);
        } else if line.contains("|") {
            continue;
        }
        write!(classified_contents, "{}\n", classified_line).expect("Error writing to string.");
    }
    classified_contents
}

fn generate_class_for_type(definition: &str) -> String {
    let type_name = definition
        .split_whitespace()
        .nth(3)
        .expect("Invalid type declaration detected.");
    if definition.contains("&") {
        let mut token_iter = definition.split_whitespace().peekable();
        while let Some(cur) = token_iter.next() {
            let next = token_iter.peek();
            if let Some(next_value) = next {
                if *next_value == "&" {
                    return format!("export class {} extends {} {{", type_name, cur);
                }
            }
        }
    }
    format!("export class {} {{", type_name)
}

fn generate_class_for_interface(definition: &str) -> String {
    let interface_name = definition
        .split_whitespace()
        .nth(2)
        .expect("Invalid interface declaration detected.");
    format!("export class {} {{", interface_name)
}

#[test]
fn test_generate_class_for_type() {
    let one = "export declare type TestType = {";
    assert_eq!(generate_class_for_type(one), "export class TestType {");
    let two = "export declare type test_type = {";
    assert_eq!(generate_class_for_type(two), "export class test_type {");
    let three = "export declare type TestType = UnionType & {";
    assert_eq!(generate_class_for_type(three), "export class TestType extends UnionType {");
}

#[test]
fn test_generate_class_for_interface() {
    assert_eq!(generate_class_for_interface("export interface TestInterface {"), "export class TestInterface {");
}
