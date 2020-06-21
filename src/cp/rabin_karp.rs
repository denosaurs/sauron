use std::path::PathBuf;
use std::collections::HashMap;

use crate::cp::tok::Tok;
use crate::cp::frame::{Frame, FrameIterator};
use crate::cp::duplicate::Duplicate;

pub struct RabinKarp;

impl RabinKarp {
  pub fn find_duplicates(token_map: HashMap<PathBuf, Vec<Tok>>) -> Vec<Duplicate> {
    let mut store: HashMap<usize, (PathBuf, Frame)> = HashMap::new();
    let mut clones = vec![];

    let token_map: HashMap<PathBuf, FrameIterator> = token_map
      .iter()
      .map(|(path, vec)| {
        (path.to_owned(), FrameIterator::new(vec.clone(), 40))
      }).collect();

    let mut clone = None;
    let mut in_store = None;

    for (path, frames) in token_map {
      for frame in frames {
        match store.get(&frame.hash) {
          Some((from_path, from)) => {
            in_store = Some(from.clone());
            if clone.is_none() {
              clone = Some(Duplicate::new(&frame, &path, from, &from_path));
            }
          },
          None => {
            if let Some(cl) = &clone {
              clones.push(cl.clone())
            }
            clone = None;
            store.insert(frame.hash, (path.to_owned(), frame.clone()));
          }
        }
        if let Some(duplicate) = &mut clone {
          duplicate.enlarge(&frame, &in_store.clone().unwrap());
        }
      }

      if let Some(cl) = &clone {
        clones.push(cl.clone())
      }
    }

    clones
  }
}
