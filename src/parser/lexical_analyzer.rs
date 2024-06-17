
#[derive(Clone, PartialEq, Debug)]
pub enum Symbols {
    EOF,
    LeftParen(u32, u32),
    RightParen(u32, u32),
    LeftBracket(u32, u32),
    RightBracket(u32, u32),
    LeftCurly(u32, u32),
    RightCurly(u32, u32),

    Def(u32, u32),
    Defn(u32, u32),
    Doc(u32, u32),
    Let(u32, u32),
    Require(u32, u32),

    LiteralName(u32, u32, Box<String>),
    LiteralKeyword(u32, u32, Box<String>)
}

pub trait LexicalAnalyzerMethods {
    fn new(input: &'static str) -> Self;
    fn get_char(&self) -> char;
    fn advance(&mut self) -> ();

    fn is_operator_or_delimiter(&mut self) -> Option<Symbols>;
    fn is_reserved_keywords(&self, text: &str, start: u32, end: u32) -> Option<Symbols>; 

    fn get_symbol(&mut self) -> Result<Symbols, Box<String>>;
}

pub struct LexicalAnalyzer {
    buffer: Vec<char>,
    index: u32,
    stack_elements: Vec::<char>,
    parenthesis_mismatch_message: Box<String>,
    parenthesis_mismatch: bool
}

impl LexicalAnalyzerMethods for LexicalAnalyzer {

    fn new(input: &'static str) -> Self {
        LexicalAnalyzer {
            buffer: input.chars().collect(),
            index: 0,
            stack_elements: Vec::<char>::new(),
            parenthesis_mismatch_message: Box::new(String::new()),
            parenthesis_mismatch: false
        }
    }

    fn get_char(&self) -> char {
        match self.buffer.get(self.index as usize) {
			Some(x) => {
				return x.clone()
			},
			_ => '\0'
		}
    }

    fn advance(&mut self) -> () {
        self.index = self.index + 1
    }

    fn is_operator_or_delimiter(&mut self,) -> Option<Symbols> {
        let ch1 = self.get_char();
        let start = self.index;
        match &ch1 {
            '(' => {
                self.stack_elements.push(ch1);
                self.advance();
                Some(Symbols::LeftParen(start, self.index))
            },
            '[' => {
                self.stack_elements.push(ch1);
                self.advance();
                Some(Symbols::LeftBracket(start, self.index))
            },
            '{' => {
                self.stack_elements.push(ch1);
                self.advance();
                Some(Symbols::LeftCurly(start, self.index))
            },
            ')' => {
                match self.stack_elements.last() {
                    Some(x) => {
                        match &x {
                            '(' => {
                                self.stack_elements.pop();
                                self.advance();
                                Some(Symbols::RightParen(start, self.index))
                            },
                            _ => {
                                self.parenthesis_mismatch = true;
                                self.parenthesis_mismatch_message = Box::new("Closing ')' without opening '('".to_string());
                                None
                            }
                        }
                    },
                    _ => {
                        self.parenthesis_mismatch = true;
                        self.parenthesis_mismatch_message = Box::new("Closing ')' without opening parentesis".to_string());
                        None
                    }
                }
            },
            ']' => {
                match self.stack_elements.last() {
                    Some(x) => {
                        match &x {
                            '[' => {
                                self.stack_elements.pop();
                                self.advance();
                                Some(Symbols::RightBracket(start, self.index))
                            },
                            _ => {
                                self.parenthesis_mismatch = true;
                                self.parenthesis_mismatch_message = Box::new("Closing ']' without opening '['".to_string());
                                None
                            }
                        }
                    },
                    _ => {
                        self.parenthesis_mismatch = true;
                        self.parenthesis_mismatch_message = Box::new("Closing ']' without opening parentesis".to_string());
                        None
                    }
                }
            },
            '}' => {
                match self.stack_elements.last() {
                    Some(x) => {
                        match &x {
                            '{' => {
                                self.stack_elements.pop();
                                self.advance();
                                Some(Symbols::RightCurly(start, self.index))
                            },
                            _ => {
                                self.parenthesis_mismatch_message = Box::new("Closing '}' without opening '{'".to_string());
                                self.parenthesis_mismatch = true;
                                None
                            }
                        }
                    },
                    _ => {
                        self.parenthesis_mismatch = true;
                        self.parenthesis_mismatch_message = Box::new("Closing '}' without opening parentesis".to_string());
                        None
                    }
                }
            },
            _ => None
        }
    }

    fn is_reserved_keywords(&self, text: &str, start: u32, end: u32) -> Option<Symbols> {
        match text {
            "def" => Some(Symbols::Def(start, end)),
            "defn" => Some(Symbols::Defn(start, end)),
            "doc" => Some(Symbols::Doc(start, end)),
            "let" => Some(Symbols::Let(start, end)),
            "require" => Some(Symbols::Require(start, end)),
            _ => {
                match (&text.starts_with(':'), text.len() == (1 as usize)) {
                    (true, true) => None,
                    (true, false) => {
                        Some(Symbols::LiteralKeyword(start, end, Box::new(text.to_owned())))
                    },
                    _ => Some(Symbols::LiteralName(start, end, Box::new(text.to_owned())))
                }
            }
        }
    }

    fn get_symbol(&mut self) -> Result<Symbols, Box<String>> {

        /* Remove whitespace, lineshift and comments */
        loop {
            let ch = self.get_char();
            match ch {
                ' ' | '\t' => {
                    self.advance();
                    continue
                },
                '\r' | '\n' => {
                    self.advance();
                    continue
                },
                ';' => {
                    loop {
                        let ch = self.get_char();
                        match ch {
                            '\r' | '\n' | '\0' => break,
                            _ => self.advance()
                        }
                    }
                }
                _ => break
            } 
        }

        /*  Handle end of file */
        match self.get_char() {
            '\0' => return Ok(Symbols::EOF),
            _ => () 
        }

        let start = self.index; /* Save start position for next symbol */

        /* Check for operators or delimiters */
        let symbol = self.is_operator_or_delimiter();

        match symbol {
            Some(symb) => {
                match self.parenthesis_mismatch {
                    true => {
                        self.parenthesis_mismatch = false;
                        return Err(self.parenthesis_mismatch_message.to_owned())
                    }
                    _ => return Ok(symb)
                }
            }
            None => ()
        }

        /* Check for reserved keywords or literal names */
        if self.get_char().is_alphabetic() || self.get_char() == '_' || self.get_char() == ':'  {

            let mut buffer = std::string::String::new();
            buffer.push(self.get_char());
            self.advance();

            loop {
                let _cur = self.get_char();
                if _cur.is_alphanumeric() || _cur == '_' {
                    buffer.push(_cur);
                    self.advance();
                    continue
                }
                break
            }

            let res = self.is_reserved_keywords(buffer.as_str(), start, self.index);

            return match res {
                Some(x) => {
                    Ok(x) /* Found reserved keyword or literals */
                },
                _ => {
                    Err(Box::new("Found ':' but there is no keyword".to_string()))
                }
            }
        }

        Err(Box::new("Illegal character found in text!".to_string()))
    }
}

#[cfg(test)]
mod tests {

    use crate::parser::lexical_analyzer::{Symbols, LexicalAnalyzerMethods, LexicalAnalyzer};

    #[test]
    fn operator_or_delimiter_list_start() {

        let mut lexer = Box::new(LexicalAnalyzer::new("  ("));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::LeftParen(2, 3) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_vector_start() {

        let mut lexer = Box::new(LexicalAnalyzer::new("  ["));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::LeftBracket(2, 3) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_dictionary_or_set_start() {

        let mut lexer = Box::new(LexicalAnalyzer::new("  {"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::LeftCurly(2, 3) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_list_end() {

        let mut lexer = Box::new(LexicalAnalyzer::new("  ()"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::LeftParen(2, 3) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::RightParen(3, 4) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_vector_end() {

        let mut lexer = Box::new(LexicalAnalyzer::new("  []"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::LeftBracket(2, 3) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::RightBracket(3, 4) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_dictionary_or_set_end() {

        let mut lexer = Box::new(LexicalAnalyzer::new("  {}"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::LeftCurly(2, 3) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::RightCurly(3, 4) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn operator_or_delimiter_multiple() {

        let mut lexer = Box::new(LexicalAnalyzer::new("([{}])"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::LeftParen(0, 1) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::LeftBracket(1, 2) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::LeftCurly(2, 3) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::RightCurly(3, 4) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::RightBracket(4, 5) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::RightParen(5, 6) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        };
    }




    #[test]
    fn keyword_def() {

        let mut lexer = Box::new(LexicalAnalyzer::new("  def"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::Def(2, 5) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn keyword_defn() {

        let mut lexer = Box::new(LexicalAnalyzer::new("defn"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::Defn(0, 4) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn keyword_doc() {

        let mut lexer = Box::new(LexicalAnalyzer::new("doc"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::Doc(0, 3) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn keyword_let() {

        let mut lexer = Box::new(LexicalAnalyzer::new("let"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::Let(0, 3) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn keyword_require() {

        let mut lexer = Box::new(LexicalAnalyzer::new("require"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::Require(0, 7) => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }




    #[test]
    fn single_semicolon_comment() {

        let mut lexer = Box::new(LexicalAnalyzer::new("; This is a comment with single semicolon start!"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::EOF => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn double_semicolon_comment() {

        let mut lexer = Box::new(LexicalAnalyzer::new(";; This is a comment with double semicolon start!"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::EOF => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn multi_semicolon_comment() {

        let mut lexer = Box::new(LexicalAnalyzer::new(";;;;;;;;; This is a comment with multiple semicolon start!"));

        match lexer.get_symbol() { 
            Ok(x) => {
                match x {
                    Symbols::EOF => assert!(true),
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

}
