#[derive(Debug, PartialEq)]
pub enum Token {
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
pub enum TokenizeError {
    InvalidChar,
}

pub type TokenizeResult = Result<Vec<Token>, TokenizeError>;

pub fn tokenize(input: &str) -> TokenizeResult {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            ' ' | '\t' | '\n' => continue,
            '#' => {
                let mut comment = String::new();
                while let Some(&c) = iter.peek() {
                    if c == '\n' {
                        break;
                    }
                    comment.push(c);
                    iter.next();
                }

                if let Some(Token::Comment(last_comment)) = tokens.last_mut() {
                    last_comment.push_str("\n");
                    last_comment.push_str(&comment);
                } else {
                    tokens.push(Token::Comment(comment));
                }
            }
            '=' => tokens.push(Token::Equal),
            ':' => tokens.push(Token::Colon),
            ';' => tokens.push(Token::Semicolon),
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '{' => tokens.push(Token::OpenBrace),
            '}' => tokens.push(Token::CloseBrace),
            '<' => tokens.push(Token::OpenAngleBracket),
            '>' => tokens.push(Token::CloseAngleBracket),
            '+' => {
                match iter.peek() {
                    Some(&'c') => {
                        tokens.push(Token::LangCpp);
                    }
                    Some(&'j') => {
                        tokens.push(Token::LangJava);
                    }
                    Some(&'o') => {
                        tokens.push(Token::LangObjC);
                    }
                    _ => return Err(TokenizeError::InvalidChar),
                }
                iter.next();
            }
            _ => {
                if c.is_alphabetic() || c == '_' {
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
                    match word.as_str() {
                        "enum" => tokens.push(Token::KeywordEnum),
                        "record" => tokens.push(Token::KeywordRecord),
                        "interface" => tokens.push(Token::KeywordInterface),
                        "static" => tokens.push(Token::KeywordStatic),
                        "deriving" => tokens.push(Token::KeywordDeriving),
                        "bool" => tokens.push(Token::KeywordBool),
                        "i8" => tokens.push(Token::KeywordI8),
                        "i16" => tokens.push(Token::KeywordI16),
                        "i32" => tokens.push(Token::KeywordI32),
                        "i64" => tokens.push(Token::KeywordI64),
                        "f32" => tokens.push(Token::KeywordF32),
                        "f64" => tokens.push(Token::KeywordF64),
                        "string" => tokens.push(Token::KeywordString),
                        "binary" => tokens.push(Token::KeywordBinary),
                        "date" => tokens.push(Token::KeywordDate),
                        "list" => tokens.push(Token::KeywordList),
                        "set" => tokens.push(Token::KeywordSet),
                        "map" => tokens.push(Token::KeywordMap),
                        "optional" => tokens.push(Token::KeywordOptional),
                        _ => tokens.push(Token::Identifier(word)),
                    }
                } else {
                    return Err(TokenizeError::InvalidChar);
                }
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Some(Token::Comment(" Accounts Domain API".to_string()))
        );
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("accounts_API".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::Equal));
        assert_eq!(tokens.next(), Some(Token::KeywordInterface));
        assert_eq!(tokens.next(), Some(Token::LangCpp));
        assert_eq!(tokens.next(), Some(Token::OpenBrace));

        // static create(): accounts_API;
        assert_eq!(tokens.next(), Some(Token::KeywordStatic));
        assert_eq!(tokens.next(), Some(Token::Identifier("create".to_string())));
        assert_eq!(tokens.next(), Some(Token::OpenParen));
        assert_eq!(tokens.next(), Some(Token::CloseParen));
        assert_eq!(tokens.next(), Some(Token::Colon));
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("accounts_API".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::Semicolon));

        // }
        assert_eq!(tokens.next(), Some(Token::CloseBrace));
        assert_eq!(tokens.next(), None);
    }

    #[test]
    fn test_tokenize_2() {
        let input = r#"
            get_user_groups(load_mode: load_mode): groups_response;
            delete_user(assignee_id: optional<string>): delete_user_response;
        "#;

        let mut tokens = tokenize(input).unwrap().into_iter();

        // get_user_groups(load_mode: load_mode): groups_response;
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("get_user_groups".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::OpenParen));
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("load_mode".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::Colon));
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("load_mode".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::CloseParen));
        assert_eq!(tokens.next(), Some(Token::Colon));
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("groups_response".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::Semicolon));

        // delete_user(assignee_id: optional<string>): delete_user_response;
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("delete_user".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::OpenParen));
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("assignee_id".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::Colon));
        assert_eq!(tokens.next(), Some(Token::KeywordOptional));
        assert_eq!(tokens.next(), Some(Token::OpenAngleBracket));
        assert_eq!(tokens.next(), Some(Token::KeywordString));
        assert_eq!(tokens.next(), Some(Token::CloseAngleBracket));
        assert_eq!(tokens.next(), Some(Token::CloseParen));
        assert_eq!(tokens.next(), Some(Token::Colon));
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("delete_user_response".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::Semicolon));

        assert_eq!(tokens.next(), None);
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
        assert_eq!(
            tokens.next(),
            Some(Token::Comment(format!(
                "{}\n{}",
                " Get the list of users cached by sync users plugin,",
                " fetch from remote on cache miss if fetch_from_remote_if_cache_miss is true"
            )))
        );
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier("get_cached_users".to_string()))
        );
        assert_eq!(tokens.next(), Some(Token::OpenParen));
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier(
                "fetch_from_remote_if_cache_miss".to_string()
            ))
        );
        assert_eq!(tokens.next(), Some(Token::Colon));
        assert_eq!(tokens.next(), Some(Token::KeywordBool));
        assert_eq!(tokens.next(), Some(Token::CloseParen));
        assert_eq!(tokens.next(), Some(Token::Colon));
        assert_eq!(tokens.next(), Some(Token::KeywordList));
        assert_eq!(tokens.next(), Some(Token::OpenAngleBracket));
        assert_eq!(
            tokens.next(),
            Some(Token::Identifier(
                "pb_common_UserDocument_UserDocument".to_string()
            ))
        );
        assert_eq!(tokens.next(), Some(Token::CloseAngleBracket));
        assert_eq!(tokens.next(), Some(Token::Semicolon));

        assert_eq!(tokens.next(), None);
    }
}
