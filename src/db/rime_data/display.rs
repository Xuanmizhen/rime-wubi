use super::{RimeDict, RimeDictEntry};
use std::fmt::{Display, Formatter, Result};

impl Display for RimeDictEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<entry \"{}\t{}", self.phrase, self.code)?;
        if let Some(weight) = self.weight {
            write!(f, "\t{weight}")?;
        }
        write!(f, "\">")
    }
}

impl Display for RimeDict {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.data.len() {
            0..3 => {
                write!(f, "<Rime dict [")?;
                for entry in &self.data {
                    write!(f, "{entry}, ")?;
                }
                write!(f, "]>")
            }
            len => {
                write!(f, "<Rime dict [")?;
                for entry in self.data.iter().take(3) {
                    write!(f, "{entry}, ")?;
                }
                write!(f, "... and {} more entries]>", len - 3)
            }
        }
    }
}
