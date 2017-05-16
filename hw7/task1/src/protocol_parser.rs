//! Module contains the protocol commands and function to parse a message to a protocol command.
//! In this protocol there are two commands: "PUBLISH", which contains data, and "RETRIEVE"
//! (without data).


/// Protocol Command contains two types
#[derive(Debug,PartialEq)]
pub enum Command {
    /// Publish command containing data
    Publish {
        /// Data that is to be published.
        /// Field may be empty
        data: String,
    },
    /// Retrieve command
    Retrieve,
}

/// Multiple types of errors may occur during parsing
#[derive(Debug,PartialEq)]
pub enum ParseCommandError {
    /// No valid command found
    NoCommand,
    /// No newline found
    NoNewline,
    /// Empty message
    Empty,
}

impl ParseCommandError {
    /// Describes why this ParseCommandError has occured
    pub fn description(&self) -> String {
        match *self {
            ParseCommandError::NoCommand => {
                "No valid command found in message. Command must be either 'PUBLISH' or 'RETRIEVE'."
                    .to_string()
            }
            ParseCommandError::NoNewline => {
                "No newline found in message. Command must end with '\\n'.".to_string()
            }
            ParseCommandError::Empty => "Empty message.".to_string(),
        }
    }
}

/// Function that parses a message to a Publish or Retrieve Command
///
/// param *message*: Message that is to be parsed
///
/// Return: Command, or Error if message is not a valid command
pub fn parse(message: &str) -> Result<Command, ParseCommandError> {
    let cmd = match message.split_whitespace().nth(0) {
        Some(c) => c,
        None => return Err(ParseCommandError::Empty),
    };

    // Missing 'newline' is an error, worse than a non-valid command
    if !message.contains("\n") {
        return Err(ParseCommandError::NoNewline);
    }

    let publish = "PUBLISH";
    if cmd == publish {
        // Unwrap is legal, because the split cannot be empty, because then it
        // would not contains "PUBLISH"
        let raw_data = message.split("\n").next().unwrap();
        let data = &raw_data[publish.len()..].trim_left();
        Ok(Command::Publish { data: data.to_string() })
    } else if cmd == "RETRIEVE" {
        Ok(Command::Retrieve)
    } else {
        Err(ParseCommandError::NoCommand)
    }
}
