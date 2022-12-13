#[derive(Debug)]
pub enum Tokens {
    Assign,
    Colon,
    Comma,
    Dot,
    FunctionDefinition,
    In,
    If,
    Quote,
    Semi,
    While,
    Bracket(String),
    Boolean(String),
    Comment(String),
    ComparisonOperator(String),
    Operator(String),
    Identifer(String),
    StringContent(String),
    Number(String),
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
                src_code: input_code.replace('\r', "").chars().collect(),
                pointer: 0,
                tokens: vec![],
            }
        }
        fn lex(&mut self) {
            /*
                Remove every EOT character (if present) and pushes an
                EOT character for indicating when the lexer stops
            */
            self.src_code.retain(|&x| x != '\u{3}');
            self.src_code.push('\u{3}');

            /* Read through string */
            while (self.src_code.len() > self.pointer) && self.get_char(self.pointer) != '\u{3}' {
                /* Multi-line comments */

                if self.get_char(self.pointer) == '-'
                    && self.get_char(self.pointer + 1) == '-'
                    && self.get_char(self.pointer + 2) == '-'
                {
                    self.pointer += 3;
                    let mut comment: Vec<char> = vec![];
                    while !(self.get_char(self.pointer) == '-'
                        && self.get_char(self.pointer + 1) == '-'
                        && self.get_char(self.pointer + 2) == '-')
                    {
                        comment.push(self.get_char(self.pointer));
                        self.pointer += 1;
                    }
                    self.pointer += 3;

                    self.tokens.push(Tokens::Comment(comment.iter().collect()))
                }

                /* Operators */
                match self.get_char(self.pointer) {
                    /* Comments */
                    '#' => {
                        let mut comment: Vec<char> = vec![];
                        self.pointer += 1;
                        while self.get_char(self.pointer) != '\n'
                            && self.get_char(self.pointer) != '\u{3}'
                        {
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
                    ';' => self.tokens.push(Tokens::Semi),
                    ':' => self.tokens.push(Tokens::Colon),
                    _ => {}
                }

                /* Strings */
                if self.get_char(self.pointer) == '"' {
                    self.tokens.push(Tokens::Quote);

                    let mut string_text: Vec<char> = vec![];
                    self.pointer += 1;
                    while self.get_char(self.pointer) != '"' {
                        /* not terminate string if \" is found */
                        if self.get_char(self.pointer) == '\\'
                            && self.get_char(self.pointer + 1) == '"'
                        {
                            string_text.push('"');
                            self.pointer += 2;
                            continue;
                        }
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

                    /* Match keywords & identifiers */
                    let token: &str = &keyword.iter().collect::<String>()[..];

                    self.tokens.push(match token {
                        "while" => Tokens::While,
                        "true" | "false" => Tokens::Boolean(token.to_owned()),
                        "fn" => Tokens::FunctionDefinition,
                        "if" => Tokens::If,
                        "in" => Tokens::In,
                        _ => Tokens::Identifer(token.to_owned()),
                    });

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
