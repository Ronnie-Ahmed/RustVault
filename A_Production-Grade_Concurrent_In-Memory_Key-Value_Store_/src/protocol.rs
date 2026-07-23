use crate::error::VaultError;
use std::time::Duration;

#[derive(Debug,PartialEq)]
pub enum Command {
    Set{
        key:String,
        value:String,
        ttl:Option<Duration>,
    },
    Get{
        key:String,
    },
    Del{
        key:String,
    },
    Ping,
}

pub fn parse_command(line: &str) -> Result<Command, VaultError> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();

    let Some(raw_cmd) = parts.first() else {
        return Err(VaultError::UnknownCommand(String::new()));
    };

    match raw_cmd.to_ascii_uppercase().as_str() {
        "PING" => Ok(Command::Ping),

        "GET" => match parts.as_slice() {
            [_, key] => Ok(Command::Get { key: key.to_string() }),
            _ => Err(VaultError::WrongArgCount("GET".into())),
        },

        "DEL" => match parts.as_slice() {
            [_, key] => Ok(Command::Del { key: key.to_string() }),
            _ => Err(VaultError::WrongArgCount("DEL".into())),
        },

        "SET" => match parts.as_slice() {
          
            [_, key, value] => Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
                ttl: None,
            }),
            [_, key, value, ex_kw, secs] if ex_kw.eq_ignore_ascii_case("EX") => {
                let secs: u64 = secs
                    .parse()
                    .map_err(|_| VaultError::InvalidTtl(secs.to_string()))?;
                Ok(Command::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                    ttl: Some(Duration::from_secs(secs)),
                })
            }
            _ => Err(VaultError::WrongArgCount("SET".into())),
        },

        other => Err(VaultError::UnknownCommand(other.to_string())),
    }
}