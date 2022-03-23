
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut t = 0;
    let mut w = 10;
    for ch in isbn.chars() {
        match ch {
            '-' => continue, // Utilizzato nei cicli for per dire di continuare
            'X' if w == 1 => t += 10,
            '0'..='9' => t += ch.to_digit(10).unwrap() * w, // Prova a convertirlo in decimale
            _ => return false,
        }
        if w == 0 {
            return false;
        } // Se vi è una stringa troppo lunga, non rischio di far andare negativo w
        w -= 1;
    }

    t % 11 == 0 && w == 0
    /* w parte da 10; se ho meno di 10 caratteri potrebbe venire una media
    pesata modulo 11, ma non sarebbe un'ISBN valido                  */
}
