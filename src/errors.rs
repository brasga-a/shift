
use std::cell::Cell;

thread_local! {
    static HAD_ERROR: Cell<bool> = Cell::new(false);
}


// Captura o erro
pub fn had_error() -> bool {
    HAD_ERROR.with(|had_error| had_error.get())
}

// Reseta o erro
pub fn reset_error() {
    HAD_ERROR.with(|had_error| had_error.set(false));
}

// Chama o erro e usa o report para printar o erro no terminal
pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

//Printa o erro
fn report(line: usize, locate: &str, message: &str){
    eprintln!("[line: {}] Error {}: {}", line, locate, message);
    HAD_ERROR.with(|had_error| had_error.set(true))
}