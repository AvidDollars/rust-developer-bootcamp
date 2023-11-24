use anyhow::{Result as AnyResult, anyhow, bail, ensure, Context}; // Result -> Result<T, anyhow::Error>
use thiserror;

fn might_fail_anyhow(flag: bool) -> AnyResult<()> {
    if flag {
        Ok(())
    } else {
        Err(anyhow!("Fuck!")) // returns anyhow::Error with "Fuck!" message
    }    
}

fn read_file(path: &str) -> AnyResult<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file at {}", path)) // from anyhow
}

// bail     ->  immediately return anyhow error
//  a)  return Err("Fuck!");
//  b)  bail!("Fuck!");
//

// ensure   ->    assert without panicking (instead reuturns Err)
//
//

fn ensure_ex() -> AnyResult<()> {
    ensure!(false, "bad :("); // like assert!
    Ok(())
}

// ANYHOW - CHAINING:
fn task_one() -> AnyResult<()> {
    Err(anyhow!("task one failed!"))
}

fn task_two() -> AnyResult<()> {
    task_one()
        .with_context(|| "task two failed with executing task one")
}

pub fn process_tasks() {
    match task_two() {
        Ok(_) => println!("success"),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            for cause in err.chain() {
                eprintln!("Error: {err}, caused by: {:?}", cause);
            }
        }
    }
}