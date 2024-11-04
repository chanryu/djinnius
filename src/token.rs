pub const TAB_SIZE: usize = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Loc {
    line: usize,
    column: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Comment(String),   // # comment
    Equal,             // =
    Colon,             // :
    Semicolon,         // ;
    OpenParen,         // (
    CloseParen,        // )
    OpenBrace,         // {
    CloseBrace,        // }
    OpenAngleBracket,  // <
    CloseAngleBracket, // >

    DirectiveExtern,  //  @extern
    DirectiveImport,  //  @import
    FilePath(String), // "path/to/file.djinni"

    LangCpp,  // +c
    LangJava, // +j
    LangObjC, // +o

    KeywordEnum,      // enum
    KeywordRecord,    // record
    KeywordInterface, // interface
    KeywordStatic,    // static
    KeywordDeriving,  // deriving

    KeywordBool,     // bool
    KeywordI8,       // i8
    KeywordI16,      // i16
    KeywordI32,      // i32
    KeywordI64,      // i64
    KeywordF32,      // f32
    KeywordF64,      // f64
    KeywordString,   // string
    KeywordBinary,   // binary
    KeywordDate,     // date
    KeywordList,     // list
    KeywordSet,      // set
    KeywordMap,      // map
    KeywordOptional, // optional
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    loc: Loc,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    UnexpectedChar(Loc),
    InvalidFilePath(Loc),
    UnknownDirective(Loc),
    UnknownLanguage(Loc),
}

pub type TokenizeResult = Result<Vec<Token>, TokenizeError>;

pub fn tokenize(input: &str) -> TokenizeResult {
    let mut iter = input.chars().peekable();
    let mut loc = Loc::default();
    let mut tokens = Vec::new();

    macro_rules! make_token {
        ($token_kind:ident) => {
            Token {
                kind: TokenKind::$token_kind,
                loc,
            }
        };
        ($token_kind:ident($value:expr)) => {
            Token {
                kind: TokenKind::$token_kind($value.into()),
                loc,
            }
        };
    }

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

                if let Some(Token {
                    kind: TokenKind::Comment(last_comment),
                    ..
                }) = tokens.last_mut()
                {
                    last_comment.push('\n');
                    last_comment.push_str(&comment);
                } else {
                    tokens.push(make_token!(Comment(comment)));
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
                    "extern" => tokens.push(make_token!(DirectiveExtern)),
                    "import" => tokens.push(make_token!(DirectiveImport)),
                    _ => return Err(TokenizeError::UnknownDirective(loc)),
                }
                loc.column += word_len + 1;
                continue;
            }
            '"' => {
                let mut path = String::new();
                while let Some(&c) = iter.peek() {
                    match c {
                        '\n' => return Err(TokenizeError::InvalidFilePath(loc)),
                        '"' => {
                            iter.next();
                            break;
                        }
                        _ => {
                            path.push(c);
                            iter.next();
                        }
                    }
                }
                let path_len = path.len();
                tokens.push(make_token!(FilePath(path)));
                loc.column += path_len + 2;
                continue;
            }
            '=' => tokens.push(make_token!(Equal)),
            ':' => tokens.push(make_token!(Colon)),
            ';' => tokens.push(make_token!(Semicolon)),
            '(' => tokens.push(make_token!(OpenParen)),
            ')' => tokens.push(make_token!(CloseParen)),
            '{' => tokens.push(make_token!(OpenBrace)),
            '}' => tokens.push(make_token!(CloseBrace)),
            '<' => tokens.push(make_token!(OpenAngleBracket)),
            '>' => tokens.push(make_token!(CloseAngleBracket)),
            '+' => {
                match iter.peek() {
                    Some(&'c') => {
                        tokens.push(make_token!(LangCpp));
                    }
                    Some(&'j') => {
                        tokens.push(make_token!(LangJava));
                    }
                    Some(&'o') => {
                        tokens.push(make_token!(LangObjC));
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
                    "enum" => tokens.push(make_token!(KeywordEnum)),
                    "record" => tokens.push(make_token!(KeywordRecord)),
                    "interface" => tokens.push(make_token!(KeywordInterface)),
                    "static" => tokens.push(make_token!(KeywordStatic)),
                    "deriving" => tokens.push(make_token!(KeywordDeriving)),
                    "bool" => tokens.push(make_token!(KeywordBool)),
                    "i8" => tokens.push(make_token!(KeywordI8)),
                    "i16" => tokens.push(make_token!(KeywordI16)),
                    "i32" => tokens.push(make_token!(KeywordI32)),
                    "i64" => tokens.push(make_token!(KeywordI64)),
                    "f32" => tokens.push(make_token!(KeywordF32)),
                    "f64" => tokens.push(make_token!(KeywordF64)),
                    "string" => tokens.push(make_token!(KeywordString)),
                    "binary" => tokens.push(make_token!(KeywordBinary)),
                    "date" => tokens.push(make_token!(KeywordDate)),
                    "list" => tokens.push(make_token!(KeywordList)),
                    "set" => tokens.push(make_token!(KeywordSet)),
                    "map" => tokens.push(make_token!(KeywordMap)),
                    "optional" => tokens.push(make_token!(KeywordOptional)),
                    _ => tokens.push(make_token!(Identifier(word))),
                }
                loc.column += word_len;
                continue;
            }
            _ => {
                return Err(TokenizeError::UnexpectedChar(loc));
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
        ($expr:expr, Some($token_kind:ident)) => {
            assert_eq!(
                $expr,
                Some(Token {
                    kind: TokenKind::$token_kind,
                    loc: Loc::default()
                })
            );
        };
        ($expr:expr, Some($token_kind:ident($value:expr))) => {
            assert_eq!(
                $expr,
                Some(Token {
                    kind: TokenKind::$token_kind($value.into()),
                    loc: Loc::default()
                })
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
        token_eq!(tokens.next(), Some(Comment(" Accounts Domain API")));
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
    fn test_tokenize_4() {
        let input = r#"
            @extern "path/to/file.yaml"
            @import "path/to/file.djinni"
        "#;

        let mut tokens = tokenize(input).unwrap().into_iter();

        token_eq!(tokens.next(), Some(DirectiveExtern));
        token_eq!(tokens.next(), Some(FilePath("path/to/file.yaml")));

        token_eq!(tokens.next(), Some(DirectiveImport));
        token_eq!(tokens.next(), Some(FilePath("path/to/file.djinni")));

        token_eq!(tokens.next(), None);
    }

    #[test]
    fn test_tokenize_error_1() {
        let input = "abc -";

        assert_eq!(
            tokenize(input),
            Err(TokenizeError::UnexpectedChar(Loc { line: 0, column: 4 }))
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
