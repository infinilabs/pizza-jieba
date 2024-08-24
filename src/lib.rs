#![forbid(unsafe_code)]

use jieba_rs::Jieba;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::Tokenizer;
use std::borrow::Cow;

#[derive(Clone)]
pub struct JiebaTokenizer {
    jieba: Jieba,
}

impl JiebaTokenizer {
    pub fn new() -> Self {
        JiebaTokenizer {
            jieba: Jieba::new(),
        }
    }
}

impl Tokenizer for JiebaTokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> Vec<Token<'a>> {
        let mut indices = text.char_indices().collect::<Vec<_>>();
        indices.push((text.len(), '\0'));

        let orig_tokens = self
            .jieba
            .tokenize(text, jieba_rs::TokenizeMode::Search, true);
        let mut tokens = Vec::new();
        let mut position = 0;

        for token in orig_tokens {
            let start_offset = indices[token.start].0;
            let end_offset = indices[token.end].0;
            let term = Cow::Borrowed(&text[start_offset..end_offset]);

            tokens.push(Token {
                term,
                start_offset: start_offset as u32,
                end_offset: end_offset as u32,
                position,
            });
            position += 1;
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    #[test]
    fn test_jieba_tokenizing() {
        let tokenizer = JiebaTokenizer::new();

        let text = "你今天很帅！";
        let tokens = tokenizer.tokenize(text);

        println!("{:?}", tokens);

        assert_eq!(tokens.len(), 4);

        assert_eq!(tokens[0].term, Cow::Borrowed("你"));
        assert_eq!(tokens[0].start_offset, 0);
        assert_eq!(tokens[0].end_offset, 3);
        assert_eq!(tokens[0].position, 0);

        assert_eq!(tokens[1].term, Cow::Borrowed("今天"));
        assert_eq!(tokens[1].start_offset, 3);
        assert_eq!(tokens[1].end_offset, 9);
        assert_eq!(tokens[1].position, 1);

        assert_eq!(tokens[2].term, Cow::Borrowed("很帅"));
        assert_eq!(tokens[2].start_offset, 9);
        assert_eq!(tokens[2].end_offset, 15);
        assert_eq!(tokens[2].position, 2);

        assert_eq!(tokens[3].term, Cow::Borrowed("！"));
        assert_eq!(tokens[3].start_offset, 15);
        assert_eq!(tokens[3].end_offset, 18);
        assert_eq!(tokens[3].position, 3);
    }
}
