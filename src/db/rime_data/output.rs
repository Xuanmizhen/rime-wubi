use super::{Dict, DictEntry};
use std::fmt::{Display, Formatter, Result};

impl Display for DictEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}\t{}", self.phrase, self.code)?;
        if let Some(weight) = self.weight {
            write!(f, "\t{weight}")?;
        }
        Ok(())
    }
}

impl From<&Dict> for String {
    fn from(dict: &Dict) -> Self {
        dict.entries().map(|entry| format!("{entry}\n")).collect()
    }
}
