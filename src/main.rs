use core::f64;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    num,
    path::Path,
};

#[derive(Debug, Clone)]
enum Token {
    CurlOpen,
    CurlClose,
    Colon,
    QuoteStart,
    QuoteEnd,
    OpenBracket,
    CloseBracket,
    Comma,
    StringToken(String),
    NumberToken(i128),
    // True,
    // False,
    // Null,
}
#[derive(Debug)]
enum JsonVal {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<JsonVal>),
    Object(HashMap<String, JsonVal>),
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn parse(&mut self) -> Result<JsonVal, String> {
        let value = self.parse_tokens()?;

        if self.pos != self.tokens.len() {
            return Err("Unexpected tokens after JSON value".to_string());
        }

        Ok(value)
    }

    fn parse_tokens(&mut self) -> Result<JsonVal, String> {
        match self.get_token_only() {
            Some(Token::OpenBracket) => self.parse_array(),

            Some(Token::CurlOpen) => self.parse_object(),
            Some(Token::QuoteStart) => {
                let value = self.parse_string()?;
                Ok(JsonVal::String(value))
            }
            _ => {
                println!("Token: {:?}", self.get_token_only().unwrap());
                return Err("Error while parsing token".to_string());
            }
        }
    }

    fn get_token_only(&self) -> Option<&Token> {
        return self.tokens.get(self.pos);
    }
    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos)?.clone();
        self.pos += 1;
        Some(token)
    }

    fn parse_object(&mut self) -> Result<JsonVal, String> {
        match self.advance() {
            Some(Token::CurlOpen) => {}
            _ => return Err("Expected {".to_string()),
        }
        let mut object = HashMap::new();

        loop {
            if matches!(self.get_token_only(), Some(Token::CurlClose)) {
                self.advance();
                return Ok(JsonVal::Object(object));
            }
            let key = match self.parse_string() {
                Ok(string) => string,
                Err(e) => return Err(e),
            };
            match self.advance() {
                Some(Token::Colon) => {}
                _ => return Err("Expected colon after key".to_string()),
            }
            let value = match self.parse_tokens() {
                Ok(token) => token,
                Err(e) => return Err(e),
            };
            object.insert(key, value);
            match self.advance() {
                Some(Token::Comma) => {}
                Some(Token::CurlClose) => return Ok(JsonVal::Object(object)),
                _ => return Err("Expected }".to_string()),
            }
        }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        match self.advance() {
            Some(Token::QuoteStart) => {}
            _ => {
                return Err("expected opening quote".to_string());
            }
        }
        let string = match self.advance() {
            Some(Token::StringToken(str)) => str,
            _ => return Err("Expected string value".to_string()),
        };

        match self.advance() {
            Some(Token::QuoteEnd) => Ok(string),
            _ => return Err("Expected ending quote".to_string()),
        }
    }
    fn parse_array(&mut self) -> Result<JsonVal, String> {
        match self.advance() {
            Some(Token::OpenBracket) => {}
            _ => return Err("Expected [".to_string()),
        }
        let mut items = Vec::new();

        loop {
            if matches!(self.get_token_only(), Some(Token::CloseBracket)) {
                self.advance();
                return Ok(JsonVal::Array(items));
            }
            items.push(self.parse_tokens().unwrap());
            match self.advance() {
                Some(Token::Comma) => {}
                Some(Token::CloseBracket) => {
                    return Ok(JsonVal::Array(items));
                }
                _ => {
                    return Err("Expected ]".to_string());
                }
            }
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(ch) = self.next_char() {
            if ch == ' ' {
                continue;
            }
            match ch {
                '{' => tokens.push(Token::CurlOpen),
                '}' => tokens.push(Token::CurlClose),
                ':' => tokens.push(Token::Colon),
                ',' => tokens.push(Token::Comma),
                ' ' => println!("Empty space"),
                '[' => tokens.push(Token::OpenBracket),
                ']' => tokens.push(Token::CloseBracket),
                '"' => {
                    tokens.push(Token::QuoteStart);
                    tokens.push(Token::StringToken(self.consume_str()));
                    tokens.push(Token::QuoteEnd);
                }
                _ => {

                    // fix this to handle number as well
                    //                     let is_number = ch.to_string().parse::<f64>().is_ok();
                    //
                    //                     if !is_number {
                    //                         println!("Unexpeted token, {}", ch);
                    //                     } else {
                    //                         // Handling the result
                    //                         match ch.to_string().parse::<i128>() {
                    //                             Ok(n) => tokens.push(Token::NumberToken(n)),
                    //                             Err(e) => println!("Not a number: {}", e),
                    //                         }
                    //                     }
                }
            }
        }
        return tokens;
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.input[self.pos..].chars().next()?;

        self.pos += ch.len_utf8();

        Some(ch)
    }
    fn consume_str(&mut self) -> String {
        let mut ch = self.next_char().unwrap();
        let mut string = String::new();
        while ch != '"' {
            string.push(ch);
            ch = self.next_char().unwrap();
        }
        return string;
    }
}

fn main() {
    let cwd = env::current_dir().unwrap().into_string().unwrap();
    let path = Path::new(&cwd);
    let file_path = path.join("sample.json");

    let file = File::open(&file_path);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            println!("File not found error: {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let mut tokens: Vec<Token> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(li) => {
                let mut lex = Lexer::new(&li);
                let token = lex.tokenize();
                tokens.extend(token);
            }
            Err(e) => println!("Error occured in reading line Error:  {}", e),
        }
    }

    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
    };
    match parser.parse() {
        Ok(json) => println!("{:#?}", json),
        Err(error) => eprintln!("Parse error: {error}"),
    }
}
