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
fn test_empty() {
    let template = "{}";
    let expected: Value = serde_json::from_str("{}").unwrap();

    let result = generate_from_string(&String::from(template)).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_string_replacements() {
    let template = r#"{
        "gen.globals": {
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
        "gen.globals": {
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
        "gen.globals": {
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
        "gen.globals": {
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
fn test_inline_list_expand() {
    let template = r#"{
        "gen.globals": {
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
fn test_inline_object_expand() {
    let template = r#"{
        "gen.globals": {
            "foo": {
                "one": 1
            }
        },
        "obj": {
            "@@{foo}" : ""
        }
    }"#;

    let expected: Value = serde_json::from_str(
        r#"{
        "obj": {
            "one" : 1
        }
    }"#,
    )
    .unwrap();

    let result = generate_from_string(&String::from(template)).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_inline_object_expand_priorities() {
    let template = r#"{
        "gen.globals": {
            "foo": {
                "one": 1
            }
        },
        "obj": {
            "one" : 2,
            "@@{foo}" : ""
        }
    }"#;

    let expected: Value = serde_json::from_str(
        r#"{
        "obj": {
            "one" : 2
        }
    }"#,
    )
    .unwrap();

    let result = generate_from_string(&String::from(template)).unwrap();
    assert_eq!(result, expected);
}

#[test]
#[cfg(target_os = "linux")]
fn test_gen_os() {
    let template = r#"{
        "gen.globals": {
            "foo": {
                "one": 1,
                "gen.os": ["windows"]
            },

           "bar" : {
                "linux only global": 1,
                "gen.os": ["linux"]
            },

            "bar2" : {
                "windows only global": 2,
                "gen.os": ["windows"],
                "a" : 2,
                "b" : 3
            }
        },
        "obj": {
            "a" : "@{foo}"
        },
        "obj2": {
            "@@{foo}" : ""
        },
        "obj3" : {
            "gen.os": ["linux"],
            "l1" : [
                {}, {},
                { "gen.os" : ["linux"], "b" : 1 },
                { "gen.os" : ["windows"] }
            ]
        },
        "obj4" : {
            "gen.os": ["windows"]
        },
        "obj5" : {
            "l1" : [
                "@{bar}", "@{bar2}"
            ]
        },
        "obj6" : {
            "@@{bar2}" : "",
            "a" : 1
        }
    }"#;

    let expected: Value = serde_json::from_str(
        r#"{
        "obj": {},
        "obj2": {},
        "obj3": {
            "l1": [
                {},
                {},
                {
                    "b": 1
                }
            ]
        },
        "obj5" : {
            "l1" : [
                {
                    "linux only global": 1
                }
            ]
        },
        "obj6" : {
            "a" : 1
        }
    }"#,
    )
    .unwrap();

    let result = generate_from_string(&String::from(template)).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_token_kind() {
    assert_eq!(
        token_kind_from_str("@{key}"),
        TokenKind::Nested("key".to_string())
    );
    assert_eq!(
        token_kind_from_str("@@{key}"),
        TokenKind::Inplace("key".to_string())
    );
    assert_eq!(token_kind_from_str("@{}"), TokenKind::None);
    assert_eq!(token_kind_from_str("@@{}"), TokenKind::None);
    assert_eq!(token_kind_from_str("@{"), TokenKind::None);
    assert_eq!(token_kind_from_str("key}"), TokenKind::None);
    assert_eq!(token_kind_from_str("key"), TokenKind::None);
    assert_eq!(token_kind_from_str(""), TokenKind::None);
}

/// tests is_allowed_in_os
#[test]
fn test_is_allowed_in_os() {
    if cfg!(linux) {
        let value1 = serde_json::json!({
            "gen.os": "windows"
        });

        let value2 = serde_json::json!({
            "gen.os": "linux"
        });

        assert!(!is_allowed_in_os(&value1));
        assert!(is_allowed_in_os(&value2));
    } else if cfg!(windows) {
        let value1 = serde_json::json!({
            "gen.os": "windows"
        });

        let value2 = serde_json::json!({
            "gen.os": "linux"
        });

        assert!(is_allowed_in_os(&value1));
        assert!(!is_allowed_in_os(&value2));
    }
}
