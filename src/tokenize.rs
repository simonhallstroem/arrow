pub fn tokenize(c: &str) -> Vec<String> {
    let tokens: Vec<String> = c
        .replace("\"", " \" ")
        .split('\"')
        .map(|ms| ms.to_string())
        .collect();
    let mut staged_tokens: Vec<String> = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        if i % 2 != 0 {
            staged_tokens.push(token[1..token.len() - 1].to_string());
        } else {
            let s_tokens: Vec<String> = token
                .replace("(", " ( ")
                .replace(")", " ) ")
                .split_whitespace()
                .map(|t| t.to_string())
                .collect();
            for t in s_tokens {
                staged_tokens.push(t)
            }
        }
    }
    staged_tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize() {
        let test = "(concat \"H W\" 2)";
        let exp: Vec<String> = vec!["(", "concat", "H W", "2", ")"]
            .iter()
            .map(|t| t.to_string())
            .collect();
        assert_eq!(tokenize(test), exp);
    }
}
