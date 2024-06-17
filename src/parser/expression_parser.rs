use super::lexical_analyzer::{ LexicalAnalyzer, LexicalAnalyzerMethods, Symbols };



#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Unit(Vec<Node>),
    List(u32, u32, Symbols, Vec<Node>, Symbols),
    Vec(u32, u32, Symbols, Vec<Node>, Symbols),
    Map(u32, u32, Symbols, Vec<Node>, Vec::<Node>, Symbols),
    Set(u32, u32, Symbols, Vec<Node>, Symbols)
}

pub trait ExpressionParserMethods {
    fn new(lexer: LexicalAnalyzer) -> Self;
    fn advance(&mut self);
    fn parse_start_unit(&mut self) -> Result<Node, Box<String>>;
    fn parse_unit(&mut self) -> Result<Node, Box<String>>;
    fn parse_list(&mut self) -> Result<Node, Box<String>>;
    fn parse_vector(&mut self) -> Result<Node, Box<String>>;
    fn parse_map(&mut self) -> Result<Node, Box<String>>;


}

pub struct ExpressionParser {
    lexer: LexicalAnalyzer,
    symbol: Result<Symbols, Box::<String>>
}

impl ExpressionParserMethods for ExpressionParser {

    fn new(lexer: LexicalAnalyzer) -> Self {
        ExpressionParser {
            lexer: lexer,
            symbol: Ok(Symbols::EOF)
        }
    }

    fn advance(&mut self) {
        self.symbol = self.lexer.get_symbol()
    }

    fn parse_start_unit(&mut self) -> Result<Node, Box<String>> {
        self.advance();
        self.parse_unit()
    }

    fn parse_unit(&mut self) -> Result<Node, Box<String>> {
        let mut nodes : Vec::<Node> = Vec::new();
        
        loop {
            match self.symbol.clone()? {
                Symbols::EOF => {
                    return Ok(Node::Unit(nodes))
                },
                Symbols::LeftParen(_, _) => {
                    nodes.push(self.parse_list()?)
                },
                Symbols::LeftBracket(_, _) => {
                    nodes.push(self.parse_vector()?)
                },
                Symbols::LeftCurly(_, _) => {
                    nodes.push(self.parse_map()?)
                },
                _ => return Err(Box::new("Syntax error!".to_string()))
            }
        }
    }

    fn parse_list(&mut self) -> Result<Node, Box<String>> {
        todo!()
    }

    fn parse_vector(&mut self) -> Result<Node, Box<String>> {
        todo!()
    }

    fn parse_map(&mut self) -> Result<Node, Box<String>> {
        todo!()
    }


}


///////////////////////////////////////////////////////////////////////////////
// Unittests below
///////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod tests {
    use crate::parser::{expression_parser::ExpressionParser, lexical_analyzer::{LexicalAnalyzer, LexicalAnalyzerMethods}};

    use super::{ExpressionParserMethods, Node };


    #[test]
    fn empty_unit() {
        let lexer = LexicalAnalyzer::new(";; This is an empty unit ()");
        let mut parser = ExpressionParser::new(lexer);
        let res = parser.parse_start_unit();

        match res {
            Ok(x) => {
                match x {
                    Node::Unit(lst) => {
                        assert_eq!(lst.len(), (0 as usize))
                    },
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }
}