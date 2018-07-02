pub mod run;
//pub mod log;
pub mod lex {

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LexItem {
    Str(String),
    Nop,
}

pub fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            ' ' => {
                it.next();
            }
            _ => {
                let mut s = lex_string(&mut it);
                result.push(LexItem::Str(s))
            }
        }
    }
    Ok(result)
}
use std::iter::Peekable;

#[allow(dead_code)]
fn lex_string<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> String {
    enum Qs {
        InSingleQuotes,
        InDoubleQuotes,
        HasEscape,
        NoQuotes
    }
    let mut qs = Qs::NoQuotes;
    let mut s = String::new();
    while let Some(&ch) = it.peek() { 
        match ch {
            ' ' => match qs {
                Qs::NoQuotes => { break }
                Qs::HasEscape => {
                    s.push(ch);
                    it.next();
                    qs = Qs::NoQuotes;
                }
                _ => {
                    s.push(ch);
                    it.next();
                }
            }
            '\'' => match qs {
                Qs::NoQuotes => {
                    it.next();
                    qs = Qs::InSingleQuotes;
                }
                Qs::InSingleQuotes => {
                    it.next();
                    qs = Qs::NoQuotes;
                }
                Qs::HasEscape => {
                    s.push(ch); 
                    it.next();
                    qs = Qs::NoQuotes;
                }
                _ => {
                    s.push(ch);
                    it.next();
                } 
            } 
            '"' => match qs {
                Qs::NoQuotes => {
                    it.next();
                    qs = Qs::InDoubleQuotes;
                }
                Qs::InDoubleQuotes => {
                    it.next();
                    qs = Qs::NoQuotes;
                }
                Qs::HasEscape => {
                    s.push(ch); 
                    it.next();
                    qs = Qs::NoQuotes;
                }
                _ => {
                    s.push(ch);
                    it.next();
                } 
            }      
            '\\' => match qs {
                Qs::HasEscape => {
                    s.push(ch);
                    it.next();
                    qs = Qs::NoQuotes;
                }
                _ => {
                    it.next();
                    qs = Qs::HasEscape;
                }
            }
            _ => {
                s.push(ch);
                it.next();
            }
        }
    }
    s
}
//fn lexx_quoted_string<T: Iterator<Item = char>>(ch char, it: &mut Peekable<T>) 
//      -> String  { }


#[cfg(test)]
mod test {
    use super::lex_string;

    #[test]
    fn test_lex_string() {
        let mut data = "hello' 'to\\'da\\ world".chars().into_iter().peekable();
        assert!("hello to'da world".to_string() == lex_string(&mut data));
    }
}
}
