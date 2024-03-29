// SPDX-License-Identifier: MIT

use crate::workspace::*;
use serde_json::Value;

#[test]
fn test_unknown_file() {
    let result = generate_from_file("unknown.template".to_string(), "unknown".to_string());
    assert!(result.is_err());
    match result {
        Err(Error::Io(e)) => match e.kind() {
            std::io::ErrorKind::NotFound => (),
            _ => panic!("Expected NotFound"),
        },
        _ => panic!("Expected IoError"),
    }
}

#[test]
fn test_string_replacements() {
    let template = r#"{
        "globals": {
            "name": "John Doe",
            "age": 42
        },
        "person": {
            "name": "@{name}",
            "age": "@{age}"
        }
    }"#;

    let expected: Value = serde_json::from_str(
        r#"{
        "person": {
            "name": "John Doe",
            "age": 42
        }
    }"#,
    )
    .unwrap();

    let result = generate_from_string(&String::from(template)).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_list_replacements() {
    let template = r#"{
        "globals": {
            "numbers": [1, 2, 3, 4, 5, 6, 7],
            "empty": []
        },
        "obj": {
            "l1": "@{numbers}",
            "l2": "@{empty}"
        }
    }"#;

    let expected: Value = serde_json::from_str(
        r#"{
        "obj": {
            "l1": [1, 2, 3, 4, 5, 6, 7],
            "l2": []
        }
    }"#,
    )
    .unwrap();

    let result = generate_from_string(&String::from(template)).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_obj_replacements() {
    let template = r#"{
        "globals": {
            "numbers": {
                "one": 1,
                "two": 2,
                "three": 3
            },
            "empty": {}
        },
        "obj": {
            "l1": "@{numbers}",
            "l2": "@{empty}"
        }
    }"#;

    let expected: Value = serde_json::from_str(
        r#"{
        "obj": {
            "l1": {
                "one": 1,
                "two": 2,
                "three": 3
            },
            "l2": {}
        }
    }"#,
    )
    .unwrap();

    let result = generate_from_string(&String::from(template)).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_gen_description() {
    let template = r#"{
        "globals": {
            "foo": {
                "one": 1,
                "gen.description": "This is a description"
            },
            "empty": {}
        },
        "obj": {
            "l1": "@{foo}"
        }
    }"#;

    let expected: Value = serde_json::from_str(
        r#"{
        "obj": {
            "l1": {
                "one": 1
            }
        }
    }"#,
    )
    .unwrap();

    let result = generate_from_string(&String::from(template)).unwrap();
    assert_eq!(result, expected);
}

#[test]
#[ignore]
fn test_inner_expand() {
    let template = r#"{
        "globals": {
            "foo": {
                "one": 1
            },
            "list" : [10, 20, 30]
        },
        "obj": {
            "somelist1" : [1, 2, 3, "@{list}"],
            "somelist2" : [1, 2, 3, "@@{list}"]
        }
    }"#;

    let expected: Value = serde_json::from_str(
        r#"{
        "obj": {
            "somelist1" : [1, 2, 3, [10, 20, 30]],
            "somelist2" : [1, 2, 3, 10, 20, 30]
        }
    }"#,
    )
    .unwrap();

    let result = generate_from_string(&String::from(template)).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_token_kind() {
    assert_eq!(token_kind("@{key}"), TokenKind::Nested("key".to_string()));
    assert_eq!(token_kind("@@{key}"), TokenKind::Inplace("key".to_string()));
    assert_eq!(token_kind("@{}"), TokenKind::None);
    assert_eq!(token_kind("@@{}"), TokenKind::None);
    assert_eq!(token_kind("@{"), TokenKind::None);
    assert_eq!(token_kind("key}"), TokenKind::None);
    assert_eq!(token_kind("key"), TokenKind::None);
    assert_eq!(token_kind(""), TokenKind::None);
}
