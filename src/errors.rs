use std::process::exit;
pub struct CompilError {
    message: String,
    line: usize,
}

impl CompilError {
    pub fn new(message: String, line: usize) -> CompilError {
        CompilError { message, line }
    }

    pub fn raise_now(message: String, line: usize) {
        let error = CompilError::new(message, line);
        error.raise();
    }

    pub fn raise(self) {
        eprintln!("Compilation failed with :");
        eprintln!("    Scan error at line {} -> {}", self.line, self.message);
        eprintln!("\n--- ABORTED ---");
        exit(1);
    }
}
