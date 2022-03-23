/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

    let mut check = String::new();
    let v: Vec<&str> = code.trim().split(" ").collect();
    let mut index = 0;
    let mut  i = 0;

    if v.len() != 4 {return false}

    while i < v.len() {

        let s = v[i];
        for ch in s.chars() {

            if index%2==0 {
                let mut n = ch.to_digit(10).unwrap() * 2;
                if n > 9 {
                    n = n - 9;
                }
                check.push_str(&n.to_string());
            } else {
                check.push(ch);
            }

            index += 1;
        }

        i+=1;
        check += " ";
    }

    let mut somma = 0;

    for num in check.chars() {
        match num {
            '0'..='9' => somma += num.to_digit(10).unwrap(),
            ' ' => (),
            _ => return false
        }

    }

    somma%10 == 0
}
