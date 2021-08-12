pub trait Append {
    fn append(self, s: String) -> String;
}

impl Append for String {
    fn append(self, s: String) -> String {
        let mut res = self;
        res.push_str(s.as_str());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append() {
        let s = String::from("Hello ");
        let exp = String::from("Hello World");
        assert_eq!(s.append("World".to_string()), exp);
    }
}
