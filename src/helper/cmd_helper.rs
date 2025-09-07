use std::{
    io::{Error, ErrorKind::InvalidInput},
    str::SplitWhitespace,
};

type CleanCmd<'a> = (SplitWhitespace<'a>, usize);

pub fn validate_set(argv: &CleanCmd) -> Result<(), Error> {
    if argv.1 != 3 {
        return Err(Error::new(
            InvalidInput,
            format!(
                "Invalid arguments for SET command: expected 2 args, found {}",
                argv.1 - 1
            ),
        ));
    }
    Ok(())
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
