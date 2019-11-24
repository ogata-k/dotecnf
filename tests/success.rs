use dotecnf::parser::ECnfParser;
use std::collections::HashMap;

fn success_test_helper(input: &str, result: &[(&str, Option<&str>)]) {
    let mut parser = ECnfParser::new();
    if let Err(e) = parser.load(input.as_bytes()) {
        eprintln!("Err:\t{}", e);
        eprintln!("input:\t\"{}\"", input);
        assert!(false);
    }
    let actual_ecnf: HashMap<String, Option<String>> = parser.build_ecnf();
    let mut expect_ecnf: HashMap<String, Option<String>> = HashMap::new();
    for (key, value) in result {
        if value.is_none() {
            expect_ecnf.insert(key.to_string(), None);
        } else {
            expect_ecnf.insert(key.to_string(), Some(value.unwrap().to_string()));
        }
    }
    assert_eq!(actual_ecnf, expect_ecnf);
}

#[test]
fn success_empty_input() {
    let input: &str = r"";
    let result: Vec<(&str, Option<&str>)> = vec![];
    success_test_helper(input, &result);
}

#[test]
fn success_comment_input() {
    let input: &str = r"# hoge i hoge j";
    let result: Vec<(&str, Option<&str>)> = vec![];
    success_test_helper(input, &result);
}

#[test]
fn success_key_not_empty_value_input() {
    let input: &str = r#"  HO_GE : " FU ga ""#;
    let result: Vec<(&str, Option<&str>)> = vec![("HO_GE", Some(" FU ga "))];
    success_test_helper(input, &result);
}

#[test]
fn success_new_line_input() {
    let input: &str = r#"

                HOGE: "FUGA"

                HOOO: ""

            "#;
    let result: Vec<(&str, Option<&str>)> = vec![("HOGE", Some("FUGA")), ("HOOO", Some(""))];
    success_test_helper(input, &result);
}
#[test]
fn success_key_empty_value_input() {
    let input: &str = r#"  HO_GE : """#;
    let result: Vec<(&str, Option<&str>)> = vec![("HO_GE", Some(""))];
    success_test_helper(input, &result);
}

#[test]
fn success_key_none_value_input() {
    let input: &str = r#"  HO_GE : "#;
    let result: Vec<(&str, Option<&str>)> = vec![("HO_GE", None)];
    success_test_helper(input, &result);
}

#[test]
fn success_key_value_empty_hierarchy_input() {
    let input: &str = r#"
        DB : {
        }"#;
    let result: Vec<(&str, Option<&str>)> = vec![];
    success_test_helper(input, &result);
}

#[test]
fn success_key_value_hierarchy_input() {
    let input: &str = r#"
        DB : {
            ACCOUNT_NAME :
            PASSWORD :
            PATH : ""
            DRIVER : "SQLite"
        }"#;
    let result: Vec<(&str, Option<&str>)> = vec![
        ("DB.ACCOUNT_NAME", None),
        ("DB.PASSWORD", None),
        ("DB.PATH", Some("")),
        ("DB.DRIVER", Some("SQLite")),
    ];
    success_test_helper(input, &result);
}

#[test]
fn success_key_values_many_empty_hierarchy_input() {
    let input: &str = r#"
        DB : {
            SQLITE: {
            }
            LOG_FILE: {
                DIRECTORY: "./log"
                PATH: "database.log"
            }
        }"#;
    let result: Vec<(&str, Option<&str>)> = vec![
        ("DB.LOG_FILE.DIRECTORY", Some("./log")),
        ("DB.LOG_FILE.PATH", Some("database.log")),
    ];
    success_test_helper(input, &result);
}

#[test]
fn success_key_values_many_hierarchy_input() {
    let input: &str = r#"
        DB : {
            DRIVER : "SQLite"
            SQLITE: {
                ACCOUNT_NAME : "user"
                PASSWORD : "_"3$#"
                PATH : ""
            }
            LOG_FILE: {
                DIRECTORY: "./log"
                PATH: "database.log"
            }
        }"#;
    let result: Vec<(&str, Option<&str>)> = vec![
        ("DB.DRIVER", Some("SQLite")),
        ("DB.SQLITE.ACCOUNT_NAME", Some("user")),
        ("DB.SQLITE.PASSWORD", Some("_\"3$#")),
        ("DB.SQLITE.PATH", Some("")),
        ("DB.LOG_FILE.DIRECTORY", Some("./log")),
        ("DB.LOG_FILE.PATH", Some("database.log")),
    ];
    success_test_helper(input, &result);
}

#[test]
fn success_key_values_input() {
    let input: &str = r#"
        # app version
        VERSION :  "4.2.23"

        # screen
        SCREEN:{
            # empty setting so empty string
            SC_ZERO: "日本語"
            SC_ONE:
            SC_TWO: "default"

            # value is "hoge hoge " and key is SCREEN.SC_THREE
            SC_THREE: "hoge hoge "
        }"#;
    let result: Vec<(&str, Option<&str>)> = vec![
        ("VERSION", Some("4.2.23")),
        ("SCREEN.SC_ZERO", Some("日本語")),
        ("SCREEN.SC_ONE", None),
        ("SCREEN.SC_TWO", Some("default")),
        ("SCREEN.SC_THREE", Some("hoge hoge ")),
    ];
    success_test_helper(input, &result);
}
