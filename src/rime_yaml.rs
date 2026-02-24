use std::io::{BufRead, Result};

fn wait_for(rdr: &mut impl BufRead, target: &str) -> Result<Option<()>> {
    let mut buf = String::with_capacity(100);
    while rdr.read_line(&mut buf)? != 0 {
        if buf == target {
            return Ok(Some(()));
        }
        buf.clear();
    }
    Ok(None)
}

pub(crate) fn skip_until_dict_data(rdr: &mut impl BufRead) -> Result<Option<()>> {
    if wait_for(rdr, "---\n")?.is_some() {
        wait_for(rdr, "...\n")
    } else {
        Ok(None)
    }
}
