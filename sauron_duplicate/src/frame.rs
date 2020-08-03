use std::ops::Range;

use crate::tok::Tok;

type Fingerprint = usize;

#[derive(Clone, Debug)]
pub struct Frame {
  pub hash: usize,
  pub start: Tok,
  pub end: Tok,
}

#[derive(Clone, Debug)]
pub struct FrameIterator {
  tokens: Vec<Tok>,
  min_tokens: usize,
  hash_pow: usize,
  hash: usize,
  position: usize,
}

impl Iterator for FrameIterator {
  type Item = Frame;
  fn next(&mut self) -> Option<Self::Item> {
    if self.tokens.len() < self.min_tokens {
      return None;
    }

    if self.position <= self.tokens.len() - self.min_tokens {
      self.position += 1;
      let slice = &self.tokens[self.range()];

      if self.position == 1 {
        self.hash = self.compute_hash(slice);
      } else {
        let range = self.range();
        let old = &self.tokens[range.start - 1];
        let new = &self.tokens[range.end - 1];
        self.hash = self.update_hash(self.hash, old, new);
      }

      return Some(Frame {
        hash: self.hash,
        start: slice[0].clone(),
        end: slice[self.min_tokens - 1].clone(),
      });
    }
    None
  }
}

impl FrameIterator {
  pub fn new(tokens: Vec<Tok>, min_tokens: usize) -> Self {
    let mut hash_pow = 1usize;
    for _ in 1..min_tokens {
      hash_pow = hash_pow.wrapping_shl(1);
    }
    Self {
      tokens,
      min_tokens,
      hash_pow,
      hash: 0,
      position: 0,
    }
  }

  fn range(&self) -> Range<usize> {
    Range {
      start: self.position - 1,
      end: self.position + self.min_tokens - 1,
    }
  }

  fn compute_hash(&self, token_hashes: &[Tok]) -> Fingerprint {
    let mut hash = 0usize;
    for h in token_hashes {
      hash = hash.wrapping_shl(1).wrapping_add(h.hash as usize);
    }
    hash
  }

  fn update_hash(
    &self,
    prev: Fingerprint,
    old: &Tok,
    new: &Tok,
  ) -> Fingerprint {
    prev
      .wrapping_sub((old.hash as usize).wrapping_mul(self.hash_pow))
      .wrapping_shl(1)
      .wrapping_add(new.hash as usize)
  }
}
