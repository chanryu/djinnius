pub const TAB_SIZE: usize = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Loc {
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub enum Token {
    Identifier(String, Loc),
    Comment(String, Loc),   // # comment
    Equal(Loc),             // =
    Colon(Loc),             // :
    Semicolon(Loc),         // ;
    OpenParen(Loc),         // (
    CloseParen(Loc),        // )
    OpenBrace(Loc),         // {
    CloseBrace(Loc),        // }
    OpenAngleBracket(Loc),  // <
    CloseAngleBracket(Loc), // >

    DirectiveExtern(Loc), //  @extern
    DirectiveImport(Loc), //  @import

    LangCpp(Loc),  // +c
    LangJava(Loc), // +j
    LangObjC(Loc), // +o

    KeywordEnum(Loc),      // enum
    KeywordRecord(Loc),    // record
    KeywordInterface(Loc), // interface
    KeywordStatic(Loc),    // static
    KeywordDeriving(Loc),  // deriving

    KeywordBool(Loc),     // bool
    KeywordI8(Loc),       // i8
    KeywordI16(Loc),      // i16
    KeywordI32(Loc),      // i32
    KeywordI64(Loc),      // i64
    KeywordF32(Loc),      // f32
    KeywordF64(Loc),      // f64
    KeywordString(Loc),   // string
    KeywordBinary(Loc),   // binary
    KeywordDate(Loc),     // date
    KeywordList(Loc),     // list
    KeywordSet(Loc),      // set
    KeywordMap(Loc),      // map
    KeywordOptional(Loc), // optional
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Identifier(a, _), Token::Identifier(b, _)) => a == b,
            (Token::Comment(a, _), Token::Comment(b, _)) => a == b,
            (Token::Equal(_), Token::Equal(_)) => true,
            (Token::Colon(_), Token::Colon(_)) => true,
            (Token::Semicolon(_), Token::Semicolon(_)) => true,
            (Token::OpenParen(_), Token::OpenParen(_)) => true,
            (Token::CloseParen(_), Token::CloseParen(_)) => true,
            (Token::OpenBrace(_), Token::OpenBrace(_)) => true,
            (Token::CloseBrace(_), Token::CloseBrace(_)) => true,
            (Token::OpenAngleBracket(_), Token::OpenAngleBracket(_)) => true,
            (Token::CloseAngleBracket(_), Token::CloseAngleBracket(_)) => true,
            (Token::DirectiveExtern(_), Token::DirectiveExtern(_)) => true,
            (Token::DirectiveImport(_), Token::DirectiveImport(_)) => true,
            (Token::LangCpp(_), Token::LangCpp(_)) => true,
            (Token::LangJava(_), Token::LangJava(_)) => true,
            (Token::LangObjC(_), Token::LangObjC(_)) => true,
            (Token::KeywordEnum(_), Token::KeywordEnum(_)) => true,
            (Token::KeywordRecord(_), Token::KeywordRecord(_)) => true,
            (Token::KeywordInterface(_), Token::KeywordInterface(_)) => true,
            (Token::KeywordStatic(_), Token::KeywordStatic(_)) => true,
            (Token::KeywordDeriving(_), Token::KeywordDeriving(_)) => true,
            (Token::KeywordBool(_), Token::KeywordBool(_)) => true,
            (Token::KeywordI8(_), Token::KeywordI8(_)) => true,
            (Token::KeywordI16(_), Token::KeywordI16(_)) => true,
            (Token::KeywordI32(_), Token::KeywordI32(_)) => true,
            (Token::KeywordI64(_), Token::KeywordI64(_)) => true,
            (Token::KeywordF32(_), Token::KeywordF32(_)) => true,
            (Token::KeywordF64(_), Token::KeywordF64(_)) => true,
            (Token::KeywordString(_), Token::KeywordString(_)) => true,
            (Token::KeywordBinary(_), Token::KeywordBinary(_)) => true,
            (Token::KeywordDate(_), Token::KeywordDate(_)) => true,
            (Token::KeywordList(_), Token::KeywordList(_)) => true,
            (Token::KeywordSet(_), Token::KeywordSet(_)) => true,
            (Token::KeywordMap(_), Token::KeywordMap(_)) => true,
            (Token::KeywordOptional(_), Token::KeywordOptional(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    InvalidChar(Loc),
    UnknownDirective(Loc),
    UnknownLanguage(Loc),
}

pub type TokenizeResult = Result<Vec<Token>, TokenizeError>;

pub fn tokenize(input: &str) -> TokenizeResult {
    let mut iter = input.chars().peekable();
    let mut loc = Loc::default();
    let mut tokens = Vec::new();

    while let Some(c) = iter.next() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                if c == '\n' {
                    loc.line += 1;
                    loc.column = 0;
                } else {
                    loc.column += if c == '\t' { TAB_SIZE } else { 1 };
                }
                continue;
            }
            '#' => {
                let mut comment = String::new();
                while let Some(&c) = iter.peek() {
                    if c == '\n' {
                        break;
                    }
                    comment.push(c);
                    iter.next();
                }

                if let Some(Token::Comment(last_comment, _)) = tokens.last_mut() {
                    last_comment.push_str("\n");
                    last_comment.push_str(&comment);
                } else {
                    tokens.push(Token::Comment(comment, loc));
                }
                loc.line += 1;
                loc.column = 0;
                continue;
            }
            '@' => {
                // read a word after '@'
                let mut word = String::new();
                while let Some(&c) = iter.peek() {
                    if c.is_alphabetic() || c == '_' {
                        word.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                let word_len = word.len();
                match word.as_str() {
                    "extern" => tokens.push(Token::DirectiveExtern(loc)),
                    "import" => tokens.push(Token::DirectiveImport(loc)),
                    _ => return Err(TokenizeError::UnknownDirective(loc)),
                }
                loc.column += word_len + 1;
                continue;
            }
            '=' => tokens.push(Token::Equal(loc)),
            ':' => tokens.push(Token::Colon(loc)),
            ';' => tokens.push(Token::Semicolon(loc)),
            '(' => tokens.push(Token::OpenParen(loc)),
            ')' => tokens.push(Token::CloseParen(loc)),
            '{' => tokens.push(Token::OpenBrace(loc)),
            '}' => tokens.push(Token::CloseBrace(loc)),
            '<' => tokens.push(Token::OpenAngleBracket(loc)),
            '>' => tokens.push(Token::CloseAngleBracket(loc)),
            '+' => {
                match iter.peek() {
                    Some(&'c') => {
                        tokens.push(Token::LangCpp(loc));
                    }
                    Some(&'j') => {
                        tokens.push(Token::LangJava(loc));
                    }
                    Some(&'o') => {
                        tokens.push(Token::LangObjC(loc));
                    }
                    _ => return Err(TokenizeError::UnknownLanguage(loc)),
                }
                iter.next();
                loc.column += 2;
                continue;
            }
            _ if c.is_alphabetic() || c == '_' => {
                let mut word = String::new();
                word.push(c);
                while let Some(&c) = iter.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        word.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }

                let word_len = word.len();
                match word.as_str() {
                    "enum" => tokens.push(Token::KeywordEnum(loc)),
                    "record" => tokens.push(Token::KeywordRecord(loc)),
                    "interface" => tokens.push(Token::KeywordInterface(loc)),
                    "static" => tokens.push(Token::KeywordStatic(loc)),
                    "deriving" => tokens.push(Token::KeywordDeriving(loc)),
                    "bool" => tokens.push(Token::KeywordBool(loc)),
                    "i8" => tokens.push(Token::KeywordI8(loc)),
                    "i16" => tokens.push(Token::KeywordI16(loc)),
                    "i32" => tokens.push(Token::KeywordI32(loc)),
                    "i64" => tokens.push(Token::KeywordI64(loc)),
                    "f32" => tokens.push(Token::KeywordF32(loc)),
                    "f64" => tokens.push(Token::KeywordF64(loc)),
                    "string" => tokens.push(Token::KeywordString(loc)),
                    "binary" => tokens.push(Token::KeywordBinary(loc)),
                    "date" => tokens.push(Token::KeywordDate(loc)),
                    "list" => tokens.push(Token::KeywordList(loc)),
                    "set" => tokens.push(Token::KeywordSet(loc)),
                    "map" => tokens.push(Token::KeywordMap(loc)),
                    "optional" => tokens.push(Token::KeywordOptional(loc)),
                    _ => tokens.push(Token::Identifier(word, loc)),
                }
                loc.column += word_len;
                continue;
            }
            _ => {
                return Err(TokenizeError::InvalidChar(loc));
            }
        }

        loc.column += 1;
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! token_eq {
        ($expr:expr, None) => {
            assert_eq!($expr, None);
        };
        ($expr:expr, Some($token_case:ident)) => {
            assert_eq!($expr, Some(Token::$token_case(Loc::default())));
        };
        ($expr:expr, Some($token_case:ident($value:expr))) => {
            assert_eq!(
                $expr,
                Some(Token::$token_case($value.into(), Loc::default()))
            );
        };
    }

    #[test]
    fn test_tokenize_1() {
        let input = r#"
            # Accounts Domain API
            accounts_API = interface +c {
                static create(): accounts_API;
            }
        "#;

        let mut tokens = tokenize(input).unwrap().into_iter();

        // # Accounts Domain API
        // accounts_API = interface +c {
        assert_eq!(
            tokens.next(),
            Some(Token::Comment(
                " Accounts Domain API".to_string(),
                Loc::default()
            ))
        );
        token_eq!(tokens.next(), Some(Identifier("accounts_API")));
        token_eq!(tokens.next(), Some(Equal));
        token_eq!(tokens.next(), Some(KeywordInterface));
        token_eq!(tokens.next(), Some(LangCpp));
        token_eq!(tokens.next(), Some(OpenBrace));

        // static create(): accounts_API;
        token_eq!(tokens.next(), Some(KeywordStatic));
        token_eq!(tokens.next(), Some(Identifier("create")));
        token_eq!(tokens.next(), Some(OpenParen));
        token_eq!(tokens.next(), Some(CloseParen));
        token_eq!(tokens.next(), Some(Colon));
        token_eq!(tokens.next(), Some(Identifier("accounts_API")));
        token_eq!(tokens.next(), Some(Semicolon));

        // }
        token_eq!(tokens.next(), Some(CloseBrace));
        token_eq!(tokens.next(), None);
    }

    #[test]
    fn test_tokenize_2() {
        let input = r#"
            get_user_groups(load_mode: load_mode): groups_response;
            delete_user(assignee_id: optional<string>): delete_user_response;
        "#;

        let mut tokens = tokenize(input).unwrap().into_iter();

        // get_user_groups(load_mode: load_mode): groups_response;
        token_eq!(tokens.next(), Some(Identifier("get_user_groups")));
        token_eq!(tokens.next(), Some(OpenParen));
        token_eq!(tokens.next(), Some(Identifier("load_mode")));
        token_eq!(tokens.next(), Some(Colon));
        token_eq!(tokens.next(), Some(Identifier("load_mode".to_string())));
        token_eq!(tokens.next(), Some(CloseParen));
        token_eq!(tokens.next(), Some(Colon));
        token_eq!(tokens.next(), Some(Identifier("groups_response")));
        token_eq!(tokens.next(), Some(Semicolon));

        // delete_user(assignee_id: optional<string>): delete_user_response;
        token_eq!(tokens.next(), Some(Identifier("delete_user")));
        token_eq!(tokens.next(), Some(OpenParen));
        token_eq!(tokens.next(), Some(Identifier("assignee_id")));
        token_eq!(tokens.next(), Some(Colon));
        token_eq!(tokens.next(), Some(KeywordOptional));
        token_eq!(tokens.next(), Some(OpenAngleBracket));
        token_eq!(tokens.next(), Some(KeywordString));
        token_eq!(tokens.next(), Some(CloseAngleBracket));
        token_eq!(tokens.next(), Some(CloseParen));
        token_eq!(tokens.next(), Some(Colon));
        token_eq!(tokens.next(), Some(Identifier("delete_user_response")));
        token_eq!(tokens.next(), Some(Semicolon));

        token_eq!(tokens.next(), None);
    }

    #[test]
    fn test_tokenize_3() {
        let input = r#"
            # Get the list of users cached by sync users plugin,
            # fetch from remote on cache miss if fetch_from_remote_if_cache_miss is true
            get_cached_users(fetch_from_remote_if_cache_miss: bool): list<pb_common_UserDocument_UserDocument>;
        "#;

        let mut tokens = tokenize(input).unwrap().into_iter();

        // # Get the list of users cached by sync users plugin,
        // # fetch from remote on cache miss if fetch_from_remote_if_cache_miss is true
        // get_cached_users(fetch_from_remote_if_cache_miss: bool): list<pb_common_UserDocument_UserDocument>;
        token_eq!(
            tokens.next(),
            Some(Comment(format!(
                "{}\n{}",
                " Get the list of users cached by sync users plugin,",
                " fetch from remote on cache miss if fetch_from_remote_if_cache_miss is true"
            )))
        );
        token_eq!(tokens.next(), Some(Identifier("get_cached_users")));
        token_eq!(tokens.next(), Some(OpenParen));
        token_eq!(
            tokens.next(),
            Some(Identifier("fetch_from_remote_if_cache_miss"))
        );
        token_eq!(tokens.next(), Some(Colon));
        token_eq!(tokens.next(), Some(KeywordBool));
        token_eq!(tokens.next(), Some(CloseParen));
        token_eq!(tokens.next(), Some(Colon));
        token_eq!(tokens.next(), Some(KeywordList));
        token_eq!(tokens.next(), Some(OpenAngleBracket));
        token_eq!(
            tokens.next(),
            Some(Identifier("pb_common_UserDocument_UserDocument"))
        );
        token_eq!(tokens.next(), Some(CloseAngleBracket));
        token_eq!(tokens.next(), Some(Semicolon));

        token_eq!(tokens.next(), None);
    }

    #[test]
    fn test_tokenize_error_1() {
        let input = "abc -";

        assert_eq!(
            tokenize(input),
            Err(TokenizeError::InvalidChar(Loc { line: 0, column: 4 }))
        );
    }

    #[test]
    fn test_tokenize_error_2() {
        let input = "xyz\n+x";

        assert_eq!(
            tokenize(input),
            Err(TokenizeError::UnknownLanguage(Loc { line: 1, column: 0 }))
        );
    }

    #[test]
    fn test_tokenize_error_3() {
        let input = "@unknown";

        assert_eq!(
            tokenize(input),
            Err(TokenizeError::UnknownDirective(Loc { line: 0, column: 0 }))
        );
    }
}
