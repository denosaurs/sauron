use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use swc_common::Span;
use swc_ecmascript::parser::token::{Token, TokenAndSpan};

#[derive(Debug, Clone)]
pub struct Tok {
  pub hash: u64,
  pub token: Token,
  pub span: Span,
}

impl From<&TokenAndSpan> for Tok {
  fn from(token_and_span: &TokenAndSpan) -> Self {
    Tok {
      hash: hash(&token_and_span.token),
      token: token_and_span.token.clone(),
      span: token_and_span.span,
    }
  }
}

impl PartialEq for Tok {
  fn eq(&self, other: &Self) -> bool {
    self.token.eq(&other.token) && !self.span.eq(&other.span)
  }
}

pub fn hash(token: &Token) -> u64 {
  let mut hasher = DefaultHasher::new();
  match token {
    Token::Word(w) => {
      "WRD".hash(&mut hasher);
      w.hash(&mut hasher)
    }
    Token::Arrow => "->".hash(&mut hasher),
    Token::Hash => "#".hash(&mut hasher),
    Token::At => "@".hash(&mut hasher),
    Token::Dot => ".".hash(&mut hasher),
    Token::DotDotDot => "...".hash(&mut hasher),
    Token::Bang => "!".hash(&mut hasher),
    Token::LParen => "(".hash(&mut hasher),
    Token::RParen => ")".hash(&mut hasher),
    Token::LBracket => "[".hash(&mut hasher),
    Token::RBracket => "]".hash(&mut hasher),
    Token::LBrace => "{".hash(&mut hasher),
    Token::RBrace => "}".hash(&mut hasher),
    Token::Semi => ";".hash(&mut hasher),
    Token::Comma => ",".hash(&mut hasher),
    Token::BackQuote => "`".hash(&mut hasher),
    Token::Template {
      raw: r,
      cooked: _,
      has_escape: _,
    } => {
      "TPL".hash(&mut hasher);
      r.hash(&mut hasher)
    }
    Token::Colon => ":".hash(&mut hasher),
    Token::ColonColon => "::".hash(&mut hasher),
    Token::BinOp(o) => {
      "BOP".hash(&mut hasher);
      o.hash(&mut hasher)
    }
    Token::AssignOp(o) => {
      "AOP".hash(&mut hasher);
      o.hash(&mut hasher)
    }
    Token::DollarLBrace => "${".hash(&mut hasher),
    Token::QuestionMark => "?".hash(&mut hasher),
    Token::PlusPlus => "++".hash(&mut hasher),
    Token::MinusMinus => "--".hash(&mut hasher),
    Token::Tilde => "~".hash(&mut hasher),
    Token::Str {
      value: v,
      has_escape: _,
    } => {
      "STR".hash(&mut hasher);
      v.hash(&mut hasher)
    }
    Token::Regex(a, b) => {
      "REG".hash(&mut hasher);
      a.hash(&mut hasher);
      b.hash(&mut hasher)
    }
    Token::Num(_) => "NUM".hash(&mut hasher), // check later, f32 is not an hashble value
    Token::BigInt(i) => {
      "BG".hash(&mut hasher);
      i.hash(&mut hasher)
    }
    Token::JSXName { name: n } => {
      "JXN".hash(&mut hasher);
      n.hash(&mut hasher)
    }
    Token::JSXText { raw: r } => {
      "JXT".hash(&mut hasher);
      r.hash(&mut hasher)
    }
    Token::JSXTagStart => {
      "JXS".hash(&mut hasher);
    }
    Token::JSXTagEnd => {
      "JXE".hash(&mut hasher);
    }
    Token::Shebang(s) => {
      "SHB".hash(&mut hasher);
      s.hash(&mut hasher)
    }
    Token::Error(_) => {
      "ERR".hash(&mut hasher);
    }
  };
  hasher.finish()
}
