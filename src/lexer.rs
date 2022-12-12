#[derive(Debug)]
pub enum Tokens {
    Comment(String),
    Quote,
    Assign,
    Dot,
    Comma,
    FunctionDefinition,
    In,
    While,
    Bracket(String),
    ComparisonOperator(String),
    Operator(String),
    StringContent(String),
    Number(String),
    Boolean(String),
    Identifer(String),
}

pub fn lex(input_code: String) -> Vec<Tokens> {
    struct Lexer {
        src_code: Vec<char>,
        pointer: usize,
        tokens: Vec<Tokens>,
    }

    impl Lexer {
        fn ini(input_code: String) -> Self {
            Self {
                src_code: input_code.replace('\r',"").chars().collect(),
                pointer: 0,
                tokens: vec![],
            }
        }
        fn lex(&mut self) {
            /* Pushes an EOL character for when the lexer stops */
            self.src_code.push('\u{0003}');

            while self.src_code.len() > self.pointer {
                if self.get_char(self.pointer) == '\u{3}' {
                    break;
                }

                /* Operators */
                match self.get_char(self.pointer) {
                    /* Comments */
                    '#' => {
                        let mut comment:Vec<char> = vec![];
                        self.pointer += 1;
                        while self.get_char(self.pointer) != '\n' && self.get_char(self.pointer) != '\u{3}' {
                            println!("{}", self.get_char(self.pointer)); 
                            comment.push(self.get_char(self.pointer)); 
                            self.pointer += 1;
                        }
                        self.tokens.push(Tokens::Comment(comment.iter().collect()))
                    }

                    '+' | '-' | '*' | '/' | '%' => {
                        /* Other operators, eg %= += */
                        if self.get_char(self.pointer + 1) == '=' {
                            self.tokens.push(Tokens::Operator(format!(
                                "{}=",
                                self.get_char(self.pointer)
                            )));
                            self.pointer += 1;
                        } else {
                            self.tokens
                                .push(Tokens::Operator(self.get_char(self.pointer).to_string()))
                        }
                    }
                    '=' => {
                        if self.get_char(self.pointer + 1) == '=' {
                            self.tokens
                                .push(Tokens::ComparisonOperator("==".to_owned()))
                        }
                        self.tokens.push(Tokens::Assign)
                    }
                    '!' | '<' | '>' => {
                        if self.get_char(self.pointer + 1) == '=' {
                            self.tokens.push(Tokens::ComparisonOperator(
                                format!("{}=", self.get_char(self.pointer)).to_owned(),
                            ))
                        } else {
                            if self.get_char(self.pointer) != '!' {
                                self.tokens.push(Tokens::ComparisonOperator(
                                    self.get_char(self.pointer).to_string(),
                                ))
                            } else {
                                self.tokens.push(Tokens::Operator('!'.to_string()))
                            }
                        }
                    }
                    '.' => self.tokens.push(Tokens::Dot),
                    ',' => self.tokens.push(Tokens::Comma),
                    '(' | ')' | '[' | ']' | '{' | '}' => self
                        .tokens
                        .push(Tokens::Bracket(self.get_char(self.pointer).to_string())),
                    _ => {}
                }

                /* Strings */
                if self.get_char(self.pointer) == '"' {
                    self.tokens.push(Tokens::Quote);

                    let mut string_text: Vec<char> = vec![];
                    self.pointer += 1;
                    while self.get_char(self.pointer) != '"' {
                        string_text.push(self.get_char(self.pointer));
                        self.pointer += 1;
                    }
                    self.tokens
                        .push(Tokens::StringContent(string_text.iter().collect()));
                    self.tokens.push(Tokens::Quote);
                }

                /* Numbers */
                if self.get_char(self.pointer).is_numeric() {
                    let mut number: Vec<char> = vec![];

                    while (self.get_char(self.pointer)).is_numeric() {
                        number.push(self.get_char(self.pointer));
                        self.pointer += 1;
                    }
                    self.tokens.push(Tokens::Number(number.iter().collect()));

                    /* To not skip commas and other symbols */
                    self.pointer -= 1;
                }

                /* Keywords */
                if self.get_char(self.pointer).is_alphabetic() {
                    let mut keyword: Vec<char> = vec![];
                    while self.get_char(self.pointer).is_alphabetic() {
                        keyword.push(self.get_char(self.pointer));
                        self.pointer += 1;
                    }

                    /* Match tokens */
                    let token: &str = &keyword.iter().collect::<String>()[..];
                    match token {
                        "true" | "false" => self.tokens.push(Tokens::Boolean(token.to_owned())),
                        "fn" => self.tokens.push(Tokens::FunctionDefinition),
                        "in" => self.tokens.push(Tokens::In),
                        "while" => self.tokens.push(Tokens::While),
                        _ => self.tokens.push(Tokens::Identifer(token.to_owned())),
                    }

                    /* To not skip commas and other symbols */
                    self.pointer -= 1;
                }
                self.pointer += 1;
            }
        }

        /* Gets nth character of the source code without consuming it */
        fn get_char(&self, n: usize) -> char {
            *self.src_code.get(n).unwrap()
        }
    }

    let mut lexer: Lexer = Lexer::ini(input_code);
    lexer.lex();
    return lexer.tokens;
}
