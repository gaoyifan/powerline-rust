use std::path;
use std::env;
use ::powerline::*;
use ::color::Color;
use ::part::*;

pub struct Cwd {
    special: &'static str,
}
impl Cwd {
    pub fn new(special: &'static str) -> Cwd {
        Cwd { special }
    }
}

fn pb_to_str(path: path::PathBuf) -> Result<String, Error> {
    Ok(path.to_str().ok_or(Error::from_str("Path is not valid UTF-8"))?.to_string())
}

impl Part for Cwd {
    fn get_segments(self) -> Result<Vec<Segment>, Error> {
        let cwd = pb_to_str(env::current_dir()?)?;
        let mut segments = Vec::new();
        let cwd_slice = if let Some(home_path) = env::home_dir() {
            let home = pb_to_str(home_path)?;
            if let Some(pos) = cwd.find(&home) {
                segments.push(Segment::simple(&format!(" {} ", self.special), Color::HOME_FG, Color::HOME_BG) );
                &cwd[pos+home.len()..]
            } else {
                cwd.as_str()
            }
        } else {
            cwd.as_str()
        };

        for val in cwd_slice.split("/").skip(1) {
            segments.push(Segment::special(&format!(" {} ", val), Color::PATH_FG, Color::PATH_BG, '\u{E0B1}', Color::SEPARATOR_FG ) );
        }
        if let Some(last) = segments.last_mut() {
            if last.val == "  " { last.val = " / ".to_owned()}
            last.fg = Color::CWD_FG;
            last.sep = '\u{E0B0}';
            last.sep_col = last.bg;
        }
        Ok(segments)
    }
}
