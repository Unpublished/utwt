use anyhow::Result;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::iter::FromIterator;
use std::path::PathBuf;
use utwt::{parse_from_path, Utmp32Parser, Utmp64Parser, UtmpEntry};

static SAMPLES_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from_iter(&[env!("CARGO_MANIFEST_DIR"), "tests", "samples"]));

fn get_basic32_expected() -> Vec<UtmpEntry> {
    vec![
        UtmpEntry::BootTime {
            kernel_version: "5.3.0-29-generic".to_owned(),
            time_in_micros: 1581199438_054727,
        },
        UtmpEntry::RunLevel {
            kernel_version: "5.3.0-29-generic".to_owned(),
            time_in_micros: 1581199447_558900,
        },
        UtmpEntry::UserProcess {
            pid: 2555,
            line: ":1".to_owned(),
            user: "upsuper".to_owned(),
            host: ":1".to_owned(),
            session: 0,
            time_in_micros: 1581199675_609322,
        },
        UtmpEntry::UserProcess {
            pid: 28885,
            line: "tty3".to_owned(),
            user: "upsuper".to_owned(),
            host: "".to_owned(),
            session: 28786,
            time_in_micros: 1581217267_195722,
        },
        UtmpEntry::LoginProcess {
            pid: 28965,
            time_in_micros: 1581217268_463588,
        },
    ]
}

fn get_basic64_expected() -> Vec<UtmpEntry> {
    vec![
        UtmpEntry::BootTime {
            kernel_version: "5.15.0-41-generic".to_owned(),
            time_in_micros: 1658083371_314869,
        },
        UtmpEntry::RunLevel {
            kernel_version: "5.15.0-41-generic".to_owned(),
            time_in_micros: 1658083400_855073,
        },
        UtmpEntry::LoginProcess {
            pid: 1219,
            time_in_micros: 1658083400_866391,
        },
    ]
}

#[test]
fn parse_basic32() -> Result<()> {
    let path = SAMPLES_PATH.join("basic32.utmp");
    let actual = Utmp32Parser::from_path(&path)?.collect::<Result<Vec<_>, _>>()?;
    let expected = get_basic32_expected();
    Ok(assert_eq!(actual, expected))
}

#[test]
fn parse_basic64() -> Result<()> {
    let path = SAMPLES_PATH.join("basic64.utmp");
    let actual = Utmp64Parser::from_path(&path)?.collect::<Result<Vec<_>, _>>()?;
    let expected = get_basic64_expected();
    Ok(assert_eq!(actual, expected))
}

#[test]
fn parse_empty() -> Result<()> {
    let path = SAMPLES_PATH.join("empty.utmp");
    let actual = parse_from_path(&path)?;
    let expected = vec![];
    Ok(assert_eq!(actual, expected))
}

#[test]
fn parse_with_partial_read() -> Result<()> {
    let path = SAMPLES_PATH.join("basic32.utmp");
    let reader = ByteReader(BufReader::new(File::open(&path)?));
    let actual = Utmp32Parser::from_reader(reader).collect::<Result<Vec<_>, _>>()?;
    let expected = get_basic32_expected();
    Ok(assert_eq!(actual, expected))
}

struct ByteReader<R>(R);

impl<R: Read> Read for ByteReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() < 1 {
            self.0.read(buf)
        } else {
            self.0.read(&mut buf[..1])
        }
    }
}
