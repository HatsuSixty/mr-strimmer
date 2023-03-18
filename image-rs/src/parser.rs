pub fn parse_request(req: String) -> Vec<String> {
    let mut request = req.clone();
    request.push(' ');
    let mut tokens: Vec<String> = Vec::new();
    let mut string = String::new();
    let mut iter = request.chars();
    while let Some(c) = iter.next() {
        match c {
            ' ' => {
                if !string.trim().is_empty() {
                    tokens.push(string.clone());
                    string = String::new();
                }
            }
            '"' => {
                while let Some(x) = iter.next() {
                    string.push(x);
                    if x == '"' {
                        break;
                    }
                }
                string.pop().unwrap();
                tokens.push(string.clone());
                string = String::new();
            }
            _   => string.push(c),
        }
    }
    tokens
}
