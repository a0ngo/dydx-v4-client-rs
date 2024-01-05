#[derive(Debug)]
pub(crate) struct ConstructorError {
    message: String,
}

impl ConstructorError {
    pub fn new(message: String) -> Self {
        ConstructorError { message }
    }
}

impl fmt::Display for ConstructorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ConstructorError {}

#[derive(Debug)]
pub(crate) struct RestError {
    message: String,
}

impl RestError {
    pub fn new(message: String) -> Self {
        RestError { message }
    }
}

impl fmt::Display for RestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for RestError {}
