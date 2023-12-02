use anyhow::Context;
use anyhow::Result as AnyResult;
use anyhow::anyhow;
use std::error::Error;

// anyhow::Result<T> -> no need to specify Error type

fn string_error(switch: bool) -> AnyResult<()> {
    match switch {
        true => Ok(()),
        false => Err(anyhow!("string error"))
    }
    
}

fn io_error(switch: bool) -> AnyResult<()> {
    match switch {
        true => Ok(()),
        false => Err(anyhow!("io error"))
    }
}

pub fn any_error() -> AnyResult<()> {
    string_error(true)?;
    io_error(false)?;
    Ok(())
}

//  OUTPUT ON ERROR:
//
//  Err(cannot parse '1a'
//
//  Caused by:
//      invalid digit found in string)
pub fn test_context(num: &str) -> AnyResult<i32> {
    let parsed: i32 = num.parse()
        .with_context(|| format!("cannot parse '{}'", num))?;
    Ok(parsed)

}