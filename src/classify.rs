pub fn classify(file_contents: &str) -> String {
    file_contents.lines().fold(String::new(), |acc, cur| {
        if cur.contains("export interface") {
            return format!("{}{}\n", acc, generate_class_for_interface(cur));
        } else if cur.contains("export declare type") && !cur.contains("|") {
            return format!("{}{}\n", acc, generate_class_for_type(cur));
        } else if cur == "};" {
            return format!("{}}}\n", acc);
        }
        format!("{}{}\n", acc, cur)
    })
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
    assert_eq!(
        generate_class_for_type(three),
        "export class TestType extends UnionType {"
    );
}

#[test]
fn test_generate_class_for_interface() {
    assert_eq!(
        generate_class_for_interface("export interface TestInterface {"),
        "export class TestInterface {"
    );
}
