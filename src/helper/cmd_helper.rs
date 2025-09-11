use std::{
    io::{Error, ErrorKind::InvalidInput},
    str::SplitWhitespace,
};

use crate::helper::cmd_opt_helper::validate_ex;

type CleanCmd<'a> = (SplitWhitespace<'a>, usize);

pub fn validate_save(argv: &CleanCmd) -> Result<(), Error> {
    if argv.1 != 1 {
        return Err(Error::new(
            InvalidInput,
            format!(
                "Invalid arguments for SET command: expected 0 args, found {}",
                argv.1 - 1
            ),
        ));
    }
    Ok(())
}

pub fn validate_set(argv: &CleanCmd) -> Result<(), Error> {
    match argv.1 {
        3 => Ok(()),
        5 => {
            let mut argv_clone = argv.0.clone();
            if let Some(opt_cmd) = argv_clone.nth(2) {
                validate_ex(opt_cmd, argv_clone.next().unwrap())
            } else {
                Err(Error::new(
                    InvalidInput,
                    "Invalid arguments for SET command\ne.g. SET key value EX 1800",
                ))
            }
        }
        _ => Err(Error::new(
            InvalidInput,
            format!(
                "Invalid arguments for SET command: expected {} args, found {}",
                if argv.1 > 3 { 4 } else { 2 },
                argv.1 - 1
            ),
        )),
    }
}

pub fn validate_get(argv: &CleanCmd) -> Result<(), Error> {
    if argv.1 != 2 {
        return Err(Error::new(
            InvalidInput,
            format!(
                "Invalid arguments for GET command: expected 1 args, found {}",
                argv.1 - 1
            ),
        ));
    }
    Ok(())
}

pub fn validate_del(argv: &CleanCmd) -> Result<(), Error> {
    if argv.1 != 2 {
        return Err(Error::new(
            InvalidInput,
            format!(
                "Invalid arguments for DEL command: expected 1 args, found {}",
                argv.1 - 1
            ),
        ));
    }
    Ok(())
}

pub fn split_cmd(cmd: &'_ str) -> CleanCmd<'_> {
    let argv = cmd.split_whitespace();
    let len = argv.clone().count();

    (argv, len)
}
