mod texts;

#[track_caller]
fn assert_snapshot(string: &str, expected: &str) {
    let json_value = crate::parse(string);

    match json_value {
        Ok(value) => {
            let mut actual = format!("{value:?}");
            if actual.len() > 120 {
                actual = format!("{value:#?}");
            }
            assert_eq!(actual, expected);
        }
        Err(error) => {
            assert_eq!(format!("{error:#}"), expected);
        }
    }
}

#[test]
fn smoke_ok() {
    assert_snapshot(
        r#"{"mare": true, "snowpity": "legendary", "cute_level": 999}"#,
        r#"Object({"cute_level": Number(999.0), "mare": Bool(true), "snowpity": String("legendary")})"#,
    );
}

#[test]
fn smoke_error() {
    assert_snapshot(
        r#"{"mare": true, "snowpity":"#,
        r#"Expected value after key "snowpity" but the string ended unexpectedly (ExpectedValue) at the end"#,
    );
}

#[test]
fn empty_complex() {
    assert_snapshot("{}", "Object({})");
    assert_snapshot("[]", "Array([])");
    assert_snapshot(
        "[[[[[[]]]]]]",
        "Array([Array([Array([Array([Array([Array([])])])])])])",
    );
    assert_snapshot(
        r#"{"a":{},"b":{},"c":{}}"#,
        r#"Object({"a": Object({}), "b": Object({}), "c": Object({})})"#,
    );
    assert_snapshot("[{}, []]", "Array([Object({}), Array([])])");
    assert_snapshot(
        r#"{"arr": [], "obj": {}}"#,
        r#"Object({"arr": Array([]), "obj": Object({})})"#,
    );
}

#[test]
fn derpibooru() {
    let response = std::fs::read_to_string(r#".\src\derpibooru_example_response.json"#).unwrap();
    let actual = texts::derpibooru_deserealized();
    assert_snapshot(&response, &actual);
}

#[test]
fn menu() {
    let response = texts::menu_string();
    let actual = texts::menu_deserealized();
    // timer start

    assert_snapshot(&response, &actual);
    // timer end
    // print spended time
}

#[test]
fn simple_literal() {
    assert_snapshot("10", "Number(10.0)");
    assert_snapshot("\"string\"", "String(\"string\")");
    assert_snapshot("true", "Bool(true)");
    assert_snapshot("false", "Bool(false)");
    assert_snapshot("null", "Null");
}

#[test]
fn object_in_object() {
    assert_snapshot(
        r#"{"mare": {"name": "fluttershy"}}"#,
        r#"Object({"mare": Object({"name": String("fluttershy")})})"#,
    );
}

#[test]
fn object_in_array() {
    assert_snapshot(
        r#"[{"mare": true}]"#,
        r#"Array([Object({"mare": Bool(true)})])"#,
    );
}

#[test]
fn error_object() {
    assert_snapshot(
        r#"{""}"#,
        r#"Expected colon after key "", but found closed curly unexpectedly (ExpectedColon) at line 1, column 4"#,
    );

    assert_snapshot(
        r#"{"string"}"#,
        r#"Expected colon after key "string", but found closed curly unexpectedly (ExpectedColon) at line 1, column 10"#,
    );

    assert_snapshot(
        r#"{"string":}"#,
        r#"Expected value after key "string" but found closed curly unexpectedly (ExpectedValue) at line 1, column 11"#,
    );

    assert_snapshot(
        r#"{"string-invalid": bbb}"#,
        r#"Expected value after key "string-invalid" found 'b' (Syntax) at line 1, column 20"#,
    );

    assert_snapshot(
        r#"{"string-num": 10,}"#,
        r#"Expected string but found trailing comma unexpectedly (TrailingComma) at line 1, column 19"#,
    );

    assert_snapshot(
        r#"{"string-num": 10}{"#,
        r#"Expected end of tokens, but found open curly unexpectedly (ExpectedEndOfFile) at line 1, column 19"#,
    )
}

#[test]
fn error_arr() {
    assert_snapshot(
        r#"[,]"#,
        r#"Expected array value or closing bracket, but found comma unexpectedly (ExpectedValue) at line 1, column 2"#,
    );

    assert_snapshot(
        r#"[10,]]"#,
        r#"Expected array value but found closed bracket unexpectedly (ExpectedValue) at line 1, column 5"#,
    );

    assert_snapshot(
        r#"[10,{]}]"#,
        r#"Expected string or closing curly, but found closed bracket unexpectedly (ExpectedKey) at line 1, column 6"#,
    );

    assert_snapshot(
        r#"["string""#,
        r#"Expected comma or closed bracket, but the string ended unexpectedly (ExpectedCommaOrClosedBracket) at the end"#,
    );

    assert_snapshot(
        r#"["string": 10]"#,
        r#"Expected comma or closed bracket, but found colon unexpectedly (ExpectedCommaOrClosedBracket) at line 1, column 10"#,
    );
}

#[test]
fn error_string() {
    assert_snapshot(
        r#""string1"#,
        r#"Expected JSON object, array or literal - missing double quote in: "string1" (Syntax) at line 1, column 8"#,
    );

    assert_snapshot(
        r#""string2\""#,
        r#"Expected JSON object, array or literal - missing double quote in: "string2"" (Syntax) at line 1, column 10"#,
    );
}

#[test]
fn error_string_unicode() {
    assert_snapshot(r#""mare \u2764""#, r#"String("mare ❤")"#);
}
