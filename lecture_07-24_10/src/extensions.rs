pub trait OnlyFirstCliArg: Iterator {
    fn only_first_provided(self) -> Option<String>
    where
        Self: Iterator<Item = String>,
        Self: Sized,
    {
        self.skip(1).nth(0)
    }
}

impl OnlyFirstCliArg for std::env::Args {}
