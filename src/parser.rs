use std::{fs, io::Read, path::PathBuf};

use anyhow::{Ok, Result};

type ByteIter<'a> = std::slice::Iter<'a, u8>;
type TokenIter<'a> = std::slice::Iter<'a, Token>;

#[derive(Debug, PartialEq, PartialOrd)]
enum Token {
    LeftBrace,  // {
    RightBrace, // }

    LeftBracket,  // [
    RightBracket, // ]

    Comma, // ,
    Colon, // :

    DoubleQuote, // "
    Space,

    String(String),
    Number(f64),
    Null,
    Boolean(bool),
}

impl Token {
    fn from_iter(reader: &mut ByteIter) -> Result<Vec<Token>> {
        let mut tokens = vec![];
        loop {
            let b = reader.next();
            if b.is_none() {
                break;
            }
            let token = match *b.unwrap() as char {
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                '[' => Token::LeftBracket,
                ']' => Token::RightBracket,
                ',' => Token::Comma,
                ':' => Token::Colon,
                '"' => {
                    let mut s = String::new();
                    while let Some(b) = reader.next() {
                        if *b as char == '"' {
                            break;
                        }
                        if *b as char == '\\' {
                            reader.next();
                        }
                        s.push(*b as char);
                    }
                    Token::String(s)
                }
                ' ' | '\n' | '\r' | '\t' => Token::Space,
                '0'..='9' | '-' => {
                    let mut num_str = String::new();
                    num_str.push(*b.unwrap() as char);
                    while let Some(&next_b) = reader.as_slice().first() {
                        let next_c = next_b as char;
                        if next_c.is_ascii_digit()
                            || next_c == '.'
                            || next_c == 'e'
                            || next_c == 'E'
                            || next_c == '+'
                            || next_c == '-'
                        {
                            num_str.push(next_c);
                            reader.next();
                        } else {
                            break;
                        }
                    }
                    let number: f64 = num_str.parse()?;
                    Token::Number(number)
                }
                't' => {
                    // true
                    reader.next();
                    reader.next();
                    reader.next();
                    Token::Boolean(true)
                }
                'f' => {
                    // false
                    reader.next();
                    reader.next();
                    reader.next();
                    reader.next();
                    Token::Boolean(false)
                }
                'n' => {
                    // null
                    reader.next();
                    reader.next();
                    reader.next();
                    Token::Null
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Invalid character for token: {}",
                        b.unwrap()
                    ));
                }
            };
            tokens.push(token);
        }
        Ok(tokens)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Ast {
    Object(Object),
    Array(Array),
}

impl Ast {
    fn from_tokens(tokens: &[Token]) -> Result<Ast> {
        let mut reader = tokens.iter();

        let b = reader.next();
        if b.is_none() {
            return Err(anyhow::anyhow!("Empty JSON"));
        }
        let token = b.unwrap();
        let ast = match token {
            Token::LeftBrace => {
                let object = Object::from_tokens(&mut reader)?;
                Ast::Object(object)
            }
            Token::LeftBracket => {
                let array = Array::from_tokens(&mut reader)?;
                Ast::Array(array)
            }
            _ => {
                return Err(anyhow::anyhow!("Invalid token: {token:?}"));
            }
        };
        println!("{:?}", ast);

        Ok(ast)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Value {
    Object(Object),
    Array(Array),
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
}

impl Value {
    fn from_tokens(token: &Token, iter: &mut TokenIter) -> Result<Value> {
        let value = match token {
            Token::LeftBrace => {
                let object = Object::from_tokens(iter)?;
                Value::Object(object)
            }
            Token::LeftBracket => {
                let array = Array::from_tokens(iter)?;
                Value::Array(array)
            }
            Token::String(s) => match s.as_str() {
                "null" => Value::Null,
                "true" => Value::Boolean(true),
                "false" => Value::Boolean(false),
                _ => Value::String(s.clone()),
            },
            Token::Number(n) => Value::Number(*n),
            Token::Null => Value::Null,
            Token::Boolean(b) => Value::Boolean(*b),
            _ => return Err(anyhow::anyhow!("Invalid token: {token:?}")),
        };
        Ok(value)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Object {
    members: Vec<ObjectMember>,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct ObjectMember {
    key: String,
    value: Value,
}

impl Object {
    fn from_tokens(reader: &mut TokenIter) -> Result<Object> {
        let mut members = vec![];
        loop {
            let b = reader.next();
            if b.is_none() {
                break;
            }
            let token = b.unwrap();
            match token {
                Token::String(s) => {
                    let key = s.clone();
                    let mut member = ObjectMember {
                        key,
                        value: Value::Null,
                    };
                    loop {
                        let b = reader.next();
                        if b.is_none() {
                            break;
                        }
                        let token = b.unwrap();
                        match token {
                            Token::Space => continue,
                            Token::Colon => continue,
                            _ => {
                                let value = Value::from_tokens(token, reader)?;
                                member.value = value;
                                break;
                            }
                        }
                    }
                    members.push(member);
                }
                Token::Space | Token::Comma => continue,
                Token::RightBrace => break,
                _ => {
                    return Err(anyhow::anyhow!(
                        "Invalid token: {token:?} for object member"
                    ));
                }
            };
        }
        Ok(Object { members })
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Array {
    members: Vec<Value>,
}

impl Array {
    fn from_tokens(reader: &mut TokenIter) -> Result<Array> {
        let mut members = vec![];
        loop {
            let b = reader.next();
            if b.is_none() {
                break;
            }
            let token = b.unwrap();
            match token {
                Token::RightBracket => break,
                Token::Space => continue,
                Token::Comma => continue,
                _ => {
                    let value = Value::from_tokens(token, reader)?;
                    members.push(value);
                }
            }
        }

        Ok(Array { members })
    }
}

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new() -> Self {
        Self { tokens: vec![] }
    }

    fn parse(&mut self, json_data: &[u8]) -> Result<()> {
        let mut reader = json_data.iter();
        let tokens = Token::from_iter(&mut reader)?;
        self.tokens = tokens;

        Ok(())
    }
}

pub fn parse_coordinate_pairs(file_path: PathBuf) -> Result<()> {
    let mut f = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let mut parser = Parser::new();
    parser.parse(&buffer)?;

    // for token in &parser.tokens {
    //     println!("{:?}", token);
    //     if token == &Token::RightBrace {
    //         break;
    //     }
    // }
    let ast = Ast::from_tokens(&parser.tokens)?;
    println!("ast: {:?}", ast);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean() {
        let json_data = b"{\"key\": true}, {\"key\": false}, {\"key\": null}";
        let mut reader = json_data.iter();
        let tokens = Token::from_iter(&mut reader).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftBrace,
                Token::String("key".to_string()),
                Token::Colon,
                Token::Space,
                Token::Boolean(true),
                Token::RightBrace,
                Token::Comma,
                Token::Space,
                Token::LeftBrace,
                Token::String("key".to_string()),
                Token::Colon,
                Token::Space,
                Token::Boolean(false),
                Token::RightBrace,
                Token::Comma,
                Token::Space,
                Token::LeftBrace,
                Token::String("key".to_string()),
                Token::Colon,
                Token::Space,
                Token::Null,
                Token::RightBrace
            ]
        );
    }
}
