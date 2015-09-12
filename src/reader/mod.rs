//! Contains high-level interface for a pull-based XML parser.
//!
//! The most important type in this module is `EventReader`, which provides an iterator
//! view for events in XML document.

use std::io::Read;

use common::{Position, TextPosition};
use self::parser::PullParser;
use self::events::XmlEvent;

pub use self::config::ParserConfig;

mod lexer;
mod parser;
pub mod config;
pub mod events;

/// A wrapper around an `std::io::Reader` which provides pull-based XML parsing.
pub struct EventReader<R: Read> {
    source: R,
    parser: PullParser
}

impl<R: Read> EventReader<R> {
    /// Creates a new reader, consuming given stream.
    #[inline]
    pub fn new(source: R) -> EventReader<R> {
        EventReader::new_with_config(source, ParserConfig::new())
    }

    /// Creates a new reader with the provded configuration, consuming given `Buffer`.
    #[inline]
    pub fn new_with_config(source: R, config: ParserConfig) -> EventReader<R> {
        EventReader { source: source, parser: PullParser::new(config) }
    }

    /// Pulls and returns next XML event from the stream.
    ///
    /// If returned event is `XmlEvent::Error` or `XmlEvent::EndDocument`, then
    /// further calls to this method will return this event again.
    #[inline]
    pub fn next(&mut self) -> XmlEvent {
        self.parser.next(&mut self.source)
    }
}

impl<B: Read> Position for EventReader<B> {
    /// Returns the position of the last event produced by the parser
    #[inline]
    fn position(&self) -> TextPosition {
        self.parser.position()
    }
}

impl<R: Read> IntoIterator for EventReader<R> {
    type Item = XmlEvent;
    type IntoIter = Events<R>;

    fn into_iter(self) -> Events<R> {
        Events { reader: self, finished: false }
    }
}

/// An iterator over XML events created from some type implementing `Read`.
///
/// When the next event is `xml::event::Error` or `xml::event::EndDocument`, then
/// it will be returned by the iterator once, and then it will stop producing events.
pub struct Events<R: Read> {
    reader: EventReader<R>,
    finished: bool
}

impl<R: Read> Events<R> {
    #[inline]
    pub fn into_inner(self) -> EventReader<R> {
        self.reader
    }
}

impl<R: Read> Iterator for Events<R> {
    type Item = XmlEvent;

    #[inline]
    fn next(&mut self) -> Option<XmlEvent> {
        if self.finished { None }
        else {
            let ev = self.reader.next();
            match ev {
                XmlEvent::EndDocument | XmlEvent::Error(_) => self.finished = true,
                _ => {}
            }
            Some(ev)
        }
    }
}

impl<'r> EventReader<&'r [u8]> {
    /// Convenience method to create a reader from a string slice.
    #[inline]
    pub fn from_str(source: &'r str) -> EventReader<&'r [u8]> {
        EventReader::new(source.as_bytes())
    }
}
