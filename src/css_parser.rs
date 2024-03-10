use css::{ Color, Declaration, Rule, Selector, SimpleSelector, Stylesheet, Unit, Value };
use std::iter::Peekable;
use std::str::Chars;

pub struct CssParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> CssParser<'a> {
    pub fn new(full_css: &str) -> CssParser {
        CssParser {
            chars: full_css.chars().peekable(),
        }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet::default(),

        while self.chars.peek().is_some() {
            let selectors = self.parse_selectors();
            let styles = self.parse_declarations();
            let rule = Rule::new(selectors, styles);

            stylesheet.rules.push(rule)
        }
        stylesheet
    }

    pub fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new(),

        while self.chars.peek().map_or(false, |c| *c != '{') {
            let selector = self.parse_selector();
            if selector != Selector::default() {
                selectors.push(selector)
            }

            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c != ',') {
                self.chars.next();
            }
        }
        self.chars.next();
        selectors
    }

    pub fn parse_selector(&mut self) -> Vec<Selector> {
        let mut sselector = SimpleSelctor::default(),
        let mut selector = Selector::default(),

    }
}