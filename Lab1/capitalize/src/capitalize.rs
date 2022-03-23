pub fn capitalize(s: &str) -> String {
    let mut s_uppercase = String::new();
    let mut i = 0;

    let v: Vec<&str> = s.trim().split(" ").collect();
    // println!("v: {:?}", v);                         /* :? stamperà als struttura di v */

    while i < v.len(){

        let mut first= 0;
        let s = v[i];

        for ch in s.chars(){
            match ch {
                ch => if first == 0 {
                    s_uppercase.push_str(&(ch.to_uppercase().to_string()));
                    first+= 1;
                } else {
                    s_uppercase.push(ch);
                },
            }
        }

        i+=1;
        s_uppercase += " ";
    }

    return s_uppercase.trim().to_string()
}