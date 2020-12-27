use stack_vec::StackVec;

use crate::console::{kprint, kprintln, CONSOLE};
use core::str;
use shim::io::{Write as IOWrite};
/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// returns if the `exit` command is called.
const BELL: u8 = 7;
const BACKSPACE: u8 = 8;
const DELETE: u8 = 127;
const CMD_LEN: usize = 512;
const ARG_LEN: usize = 64;

pub fn shell(prefix: &str) -> ! {

    kprintln!("Welcome to RustOS by ValentinB");

    loop {
        let mut cmd_buf_cap = [0u8; CMD_LEN];
        let mut arg_buf_cap = [""; ARG_LEN];

        kprint!("{}", prefix);
        let mut buf = StackVec::new(&mut cmd_buf_cap);

        'cmd: loop {
            if buf.is_full() {
                kprintln!("command length exceeds {}", CMD_LEN);
                break 'cmd;
            }

            let byte = CONSOLE.lock().read_byte();
            if byte == b'\n' || byte == b'\r' {
                kprintln!();
                let cmd =
                    str::from_utf8(buf.into_slice()).expect("can't convert byte array to string");
                match Command::parse(cmd, &mut arg_buf_cap) {
                    Err(Error::Empty) => {}
                    Err(Error::TooManyArgs) => {
                        kprintln!("error: too many arguments");
                    }
                    Ok(cmd) => {
                        process_command(cmd);
                    }
                }
                break 'cmd;
            } else {
                let mut console = CONSOLE.lock();
                if byte == BACKSPACE || byte == DELETE {
                    if buf.pop() == None {
                        console.write_byte(BELL);
                    } else {
                        console.write(&[BACKSPACE, b' ',BACKSPACE]);
                    }
                } else if byte < 32 || byte == 255 {
                    // Discard non-printable characters and send an alert.
                    console.write_byte(BELL);
                } else {
                    if buf.push(byte).is_err() {
                        console.write_byte(BELL);
                    } else {
                        console.write_byte(byte);
                    }
                }
            }
        }
    }
}

fn process_command<'a>(cmd: Command<'a>) {
    match cmd.path() {
        "echo" => {
            if cmd.args.len() > 1 {
                kprint!("{}", cmd.args[1]);
                for arg in cmd.args[2..].iter() {
                    kprint!(" {}", arg);
                }
            }
            kprintln!();
        }
        unknown => {
            kprintln!("unknown command: {}", unknown);
        }
    }
}
