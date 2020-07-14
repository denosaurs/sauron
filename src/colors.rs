use core::fmt;
use std::io::Write;

use termcolor::Color::{Ansi256, Blue, Red, Yellow};
use termcolor::{Ansi, ColorSpec, WriteColor};
#[cfg(windows)]
use termcolor::{BufferWriter, ColorChoice};

#[allow(unused)]
#[cfg(windows)]
pub fn enable_ansi() {
  BufferWriter::stdout(ColorChoice::AlwaysAnsi);
}

// pub fn bold(s: String) -> impl fmt::Display {
//   let mut style_spec = ColorSpec::new();
//   style_spec.set_bold(true);
//   style(&s, style_spec)
// }

pub fn gray(s: String) -> impl fmt::Display {
  let mut style_spec = ColorSpec::new();
  style_spec.set_fg(Some(Ansi256(8)));
  style(&s, style_spec)
}

// pub fn red(s: String) -> impl fmt::Display {
//   let mut style_spec = ColorSpec::new();
//   style_spec.set_fg(Some(Red));
//   style(&s, style_spec)
// }

pub fn red_bold(s: String) -> impl fmt::Display {
  let mut style_spec = ColorSpec::new();
  style_spec.set_fg(Some(Red)).set_bold(true);
  style(&s, style_spec)
}

pub fn yellow_bold(s: String) -> impl fmt::Display {
  let mut style_spec = ColorSpec::new();
  style_spec.set_fg(Some(Yellow)).set_bold(true);
  style(&s, style_spec)
}

pub fn blue(s: String) -> impl fmt::Display {
  let mut style_spec = ColorSpec::new();
  style_spec.set_fg(Some(Blue));
  style(&s, style_spec)
}

fn style(s: &str, colorspec: ColorSpec) -> impl fmt::Display {
  let mut v = Vec::new();
  let mut ansi_writer = Ansi::new(&mut v);
  ansi_writer.set_color(&colorspec).unwrap();
  ansi_writer.write_all(s.as_bytes()).unwrap();
  ansi_writer.reset().unwrap();
  String::from_utf8_lossy(&v).into_owned()
}
