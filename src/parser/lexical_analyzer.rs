

#[derive(Clone, PartialEq, Debug)]
pub enum Symbols {
    EOF,
    LeftParen(u32, u32),
    RightParen(u32, u32),
    LeftBracket(u32, u32),
    RightBracket(u32, u32),
    LeftCurly(u32, u32),
    RightCurly(u32, u32),
}

pub trait LexicalAnalyzerMethods {
    fn new(input: &'static str) -> Self;
    fn get_char(&self) -> char;
    fn advance(&mut self) -> ();

    fn is_operator_or_delimiter(&mut self) -> Option<Symbols>;

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

        /* Check for reserved keywords */


        Err(Box::new("".to_string()))
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

}
