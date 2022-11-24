pub enum Declare {
    Import,
    Export,
}





pub fn get_token(texts: String){
    let texts_chars = texts.as_str().chars();
    for i in texts_chars {
        if !i.is_whitespace() {
            proceed_char(i);
        }
    }
}

fn proceed_char(c: char){
    println!("{}", c);
}