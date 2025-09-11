use std::io::{Error, ErrorKind::InvalidInput};

pub fn validate_ex(opt: &str, argv: &str) -> Result<(), Error> {
    if opt != "EX" {
        return Err(Error::new(
            InvalidInput,
            format!("Invalid option: expected 'EX', found '{}'", opt),
        ));
    } else if argv.parse::<usize>().is_err() {
        return Err(Error::new(
            InvalidInput,
            "Invalid option: expected 'EX 1500'",
        ));
    }
    Ok(())
}
