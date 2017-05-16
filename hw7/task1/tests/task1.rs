extern crate task1;
use task1::protocol_parser::{ParseCommandError, Command, parse};

#[test]
fn test1_no_command() {
    // No command given
    let msg = "message without command\n";
    assert_eq!(parse(msg), Err(ParseCommandError::NoCommand));
}

#[test]
fn test2_small_letter_command() {
    // Command must be upper case
    let msg = "retrieve\n";
    assert_eq!(parse(msg), Err(ParseCommandError::NoCommand));
}

#[test]
fn test3_publish() {
    // Everything Ok
    let msg = "PUBLISH hello world\n";
    assert_eq!(parse(msg),
               Ok(Command::Publish { data: "hello world".to_string() }));
}

#[test]
fn test4_publish_mutliple_newlines1() {
    // Second line will be thrown away
    let msg = "PUBLISH hello world\n\nGruess Gott\n";
    assert_eq!(parse(msg),
               Ok(Command::Publish { data: "hello world".to_string() }));
}

#[test]
fn test5_publish_mutliple_newlines2() {
    // Two test cases in one: multiple newlines and empty data
    let msg = "PUBLISH\n\n";
    assert_eq!(parse(msg), Ok(Command::Publish { data: "".to_string() }));
}

#[test]
fn test6_publish_no_newline() {
    // Empty line
    let msg = "PUBLISH";
    assert_eq!(parse(msg), Err(ParseCommandError::NoNewline));
}

#[test]
fn test7_publish_without_space() {
    // No command found
    let msg = "PUBLISHabc\n";
    assert_eq!(parse(msg), Err(ParseCommandError::NoCommand));
}

#[test]
fn test8_retrieve() {
    // Everything Ok
    let msg = "RETRIEVE\n";
    assert_eq!(parse(msg), Ok(Command::Retrieve));
}

#[test]
fn test9_retrieve_without_newline() {
    // Retrieve without newline is an error
    let msg = "RETRIEVE";
    assert_eq!(parse(msg), Err(ParseCommandError::NoNewline));
}

#[test]
fn test10_retrieve_with_data() {
    // Data after RETRIEVE command is ignored but no error
    let msg = "RETRIEVE data must be ignored\n";
    assert_eq!(parse(msg), Ok(Command::Retrieve));
}

#[test]
fn test11_retrieve_without_space() {
    let msg = "RETRIEVEleads to error!\n";
    assert_eq!(parse(msg), Err(ParseCommandError::NoCommand));
}

#[test]
fn test12_empty1() {
    assert_eq!(parse(""), Err(ParseCommandError::Empty));
}

#[test]
fn test13_empty2() {
    assert_eq!(parse("  "), Err(ParseCommandError::Empty));
}

#[test]
fn test14_empty3() {
    assert_eq!(parse("\n"), Err(ParseCommandError::Empty));
}

#[test]
fn test15_parse_command_err_description1() {
    assert_eq!("No newline found in message. Command must end with '\\n'.".to_string(),
               ParseCommandError::NoNewline.description());
}

#[test]
fn test16_parse_command_err_description2() {
    assert_eq!("No valid command found in message. Command must be either 'PUBLISH' or 'RETRIEVE'."
                   .to_string(),
               ParseCommandError::NoCommand.description());
}

#[test]
fn test17_parse_command_err_description3() {
    assert_eq!("Empty message.".to_string(),
               ParseCommandError::Empty.description());
}
