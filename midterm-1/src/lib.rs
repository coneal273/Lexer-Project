#[derive(Debug, PartialEq, Clone)]



pub struct Token{
    pub kind: TokenKind,
    pub lexeme: String,
}


#[derive(Debug, PartialEq, Clone)]

pub enum TokenKind{
//keywords
Fn,
Let,
Return,
If,
//Else, not sure if needed
//Operators
Plus,
Minus,
Divide,
Multiply,
GreaterThan,
//Punctuation
Semicolon,
OpenCurly,
CloseCurly,
OpenParen,
CloseParen,
//Identifiers and literals
Identifier,
Integer,
Float,
String,
//Comments
Comment,
//Misc
Equal,
Other,
True,
}



pub fn lex(input: &str)-> Vec<Token>{
  let mut tokens = Vec::new();
  let mut index = 0;
  
  

  while index < input.len(){
    let c = input[index..].chars().next().unwrap();

    if c.is_whitespace(){
        index += c.len_utf8();
        continue;
    }
    

    let kind = match c {
        'a'..='z' | 'A'..='Z' => {
            let mut lexeme = String::new();
            while let Some(next_char) = input[index..].chars().next(){
                if next_char.is_ascii_alphanumeric() || next_char == '_'{
                    lexeme.push(next_char);
                    index += next_char.len_utf8();
                }else{
                    break;
                }
            }
            match lexeme.as_str(){
                "fn" => TokenKind::Fn,
                "let" => TokenKind::Let,
                "return" => TokenKind::Return,
                "if" => TokenKind::If,
                //"else" => TokenKind::Else,
                _ => TokenKind::Other,
            }
        }
        '0'..='9' =>{
            let mut lexeme = String::new();
            while let Some(next_char) = input[index..].chars().next(){
                if next_char.is_ascii_digit() || next_char == '.' {
                    lexeme.push(next_char);
                    index += next_char.len_utf8();
                }else{
                    break;
                }
            }

            if lexeme.contains('.'){
                TokenKind::Float
            }else{
                TokenKind::Integer
            }
        }

        '+' => TokenKind::Plus,
        '-' => TokenKind::Minus,
        '/' => TokenKind::Divide,
        '*' => TokenKind::Multiply,
        ';' => TokenKind::Semicolon,
        '{' => TokenKind::OpenCurly,
        '}' => TokenKind::CloseCurly,
        '(' => TokenKind::OpenParen,
        ')' => TokenKind::CloseParen,
        '=' => TokenKind::Equal,
        '/' =>{
          /*
            let mut lexeme = String::new();
            let mut lexIter = input[index..].chars();
                while let Some(next_char) = lexIter.next(){
                    if char == '\n' {
                        index += next_char.len_utf8();
                        break;
                    }
                }
                TokenKind::String
            }
*/

            if let Some('/') = input[index + 1..].chars().next(){
                let mut lexeme = String::new();
                while let Some(next_char) = input[index..].chars().next(){
                    if next_char != '\n'{
                        lexeme.push(next_char);
                        index += next_char.len_utf8();
                    }else{
                        break;
                    }
                }
                TokenKind::Comment
            }else{
                TokenKind::Other
            }
        }
        _ =>TokenKind::Other,
    };

    let token = Token{
        kind,
        lexeme: c.to_string(),
    };

    tokens.push(token);

    index += c.len_utf8();
  }

  tokens
}





fn testKeys(){
    let tokens = lex("fn");
    let passed = tokens.len() == 1 && tokens[0].kind == TokenKind::Fn;
    println!("test keywords: {}", if passed{"keywords are valid, test passed"}else{"test failed"});
    //assert_eq!(tokens[0].kind, TokenKind::Fn);
    //assert_eq!(tokens.len(), 1);
    }
fn testID(){
    let tokens = lex("foo");
    let passed = tokens.len() == 1 && tokens[0].kind == TokenKind::Identifier;
    println!("test identifier: {}", if passed {"Identifiers are valid, test passed"}else{"test failed"});
    //assert_eq!(tokens[0].kind, TokenKind::Identifier);
    //assert_eq!(tokens.len(), 1);
}

fn testInt(){
    let tokens = lex("123");
    let passed = tokens.len() == 1 && tokens[0].kind == TokenKind::Integer;
    println!("test int: {}", if passed{"Integers are valid, test passed"}else{"test failed"});
}

fn testOP(){
    let tokens = lex("/");
    let passed = tokens.len() == 1 && tokens[0].kind == TokenKind::Divide;
    println!("test operator: {}", if passed{"Operators are valid, test passed"}else{"test failed"}); 
  
}

fn testComment(){
    let tokens = lex("//This is a comment");
    let passed = tokens.len() == 1 && tokens[0].kind == TokenKind::Comment;
    println!("test comment: {}", if passed{"Comments are valid, test passed"}else{"test failed"});
   
}

fn testString(){
    let tokens = lex(r#""Anime is the supreme form of media!""#);//string literal
    let passed = tokens.len() == 1 && tokens[0].kind == TokenKind::String;
    println!("test string: {}", if passed{"Strings are valid, test passed"}else{"test failed"});
}

fn testInvalidInput(){
    let tokens = lex("@");
    let passed = tokens.len() == 1 && tokens[0].kind == TokenKind::Other;
    println!("test invalid input: {}", if passed{"Input is invalid, test passed"}else{"test failed"});
}

fn testPunc(){
    let tokens = lex(";");
    let passed = tokens.len() == 1 && tokens[0].kind == TokenKind::Semicolon;
    println!("test punctuation: {}", if passed{"Punctuation is valid, test passed"}else{"test failed"});
}

fn testFloat(){
    let tokens = lex("3.141592654");
    let passed = tokens.len() == 1 && tokens[0].kind == TokenKind::Float;
    println!("test float: {}", if passed{"Float is valid, test passed"}else{"test failed"});
}

fn testIfStatement(){
    let tokens = lex("if(x > y){return true; }");
    let validTokenKinds = vec![
        TokenKind::If,
        TokenKind::OpenParen,
        TokenKind::Identifier,
        TokenKind::GreaterThan,
        TokenKind::Identifier,
        TokenKind::CloseParen,
        TokenKind::OpenCurly,
        TokenKind::Return,
        TokenKind::True,
        TokenKind::Semicolon,
        TokenKind::CloseCurly,
    ];
    let passed = tokens.len() == validTokenKinds.len() && tokens.iter().zip(validTokenKinds).all( | (token, validTokenKinds) | token.kind == validTokenKinds);
    println!("Test if statements: {}", if passed{"valid if statements, test passed"}else{"test failed"});
}


pub fn runAllTest(){
    testKeys();
    testID();
    testInt();
    testOP();
    testComment();
    testString();
    testInvalidInput();
    testPunc();
    testFloat();
    testIfStatement();
    }




fn main(){
    runAllTest();



    


    let source_code = r#"
    fn foo(a,b,c) {
        let x = a + 1; 
        // This is a comment
        let y = bar(c - b);
        return x * y; // Add the results
    }gi
    
    fn bar(a) {
        return a - 3;
    }
    
    fn main() {
        return foo(123,56,7.89);  
    }

    fn anime(){
        let x = randNum();
        let y = randNum();
        
        if(x > y){
            println!("naruto is better than one piece)
        }
        
    }
    "#;
/*
    let tokens = lex(source_code);
    for token in tokens{
        println!("Token {{kind: {:?}, \tlexeme: \"{}\" }}",
                    format!("{:?}", token.kind), token.lexeme); */
    //}
    }






//works cited/sources used:
//https://users.rust-lang.org/t/check-string-contain-a-z/41730
//https://doc.rust-lang.org/std/primitive.char.html
//https://dev.to/kopium/simple-lexer-in-rust-b3k
//https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
