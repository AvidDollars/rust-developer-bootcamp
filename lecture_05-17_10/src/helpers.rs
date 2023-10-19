use crate::custom_errors::AppError;
use slug::slugify;
use std::collections::HashMap;

type ArgFnMap = HashMap<&'static str, fn(String) -> String>;

pub trait OnlyFirstCliArg: Iterator {
    fn only_first_provided(mut self) -> Option<String>
    where
        Self: Iterator<Item = String>,
        Self: Sized,
    {
        self.skip(1).nth(0)
    }
}

impl OnlyFirstCliArg for std::env::Args {}

pub struct TextTransformer {
    method: fn(String) -> String,
}

impl TextTransformer {
    pub fn new(cli_arg: &str) -> Result<Self, AppError> {
        let mapping = Self::get_mapping();
        let maybe_func = mapping.get(cli_arg);

        match maybe_func {
            Some(func) => Ok(Self { method: *func }),
            None => {
                let allowed: Vec<&str> = mapping.keys().copied().collect();
                let message = format!(
                    "Invalid argument: '{}'\nAllowed arguments: {}",
                    cli_arg,
                    allowed.join(" | ")
                );
                Err(AppError::InvalidCliArgument { message })
            }
        }
    }

    pub fn apply(&self, text: String) -> String {
        (self.method)(text)
    }

    fn get_mapping() -> ArgFnMap {
        let mut mapping: ArgFnMap = HashMap::new();
        mapping.insert("lowercase", |arg| arg.to_lowercase());
        mapping.insert("uppercase", |arg| arg.to_uppercase());
        mapping.insert("no-spaces", |arg| arg.replace(" ", ""));
        mapping.insert("slugify", |arg| slugify(arg));
        mapping.insert("scream", |arg| format!("{}!!!", arg));
        mapping.insert("reverse", |arg| arg.chars().rev().collect::<String>());
        mapping
    }
}
