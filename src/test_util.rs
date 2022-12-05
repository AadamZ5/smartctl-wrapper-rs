pub trait StripNewlineEnds {
    fn strip_newline_ends(&self) -> String;
}

impl StripNewlineEnds for String {
    fn strip_newline_ends(&self) -> String {
        self.strip_suffix("\r\n")
            .or_else(|| self.strip_suffix('\r'))
            .or_else(|| self.strip_suffix('\n'))
            .unwrap_or(self)
            .to_string()
    }
}
