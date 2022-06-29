pub trait ErrorReporter {
    fn error(&self, line: usize, msg: &str) {
        self.report(line, "", msg)
    }

    fn report(&self, line: usize, whre: &str, msg: &str);
}

pub struct StdoutReporter;

impl<'a> ErrorReporter for &'a StdoutReporter {
    fn report(&self, line: usize, whre: &str, msg: &str) {
        eprintln!("[line {line}] Error{whre}: {msg}")
    }
}
