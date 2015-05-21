struct Lexer<'a> {
	src: &'a [u8],
	pos: usize
}

impl<'a> Lexer<'a> {
	fn new(src: &'a [u8]) -> Lexer {
		Lexer { src: src, pos: 0 }
	}
}

impl<'a> Lexer<'a> {
	fn ch(&self) -> u8 {
		self.src[self.pos]
	}

	fn eof(&self) -> bool {
		self.pos >= self.src.len()
	}

	fn skip_ws(&mut self) -> usize {
		let prev_pos = self.pos;
		while self.valid_ws() {
			self.pos += 1;
		}
		self.pos - prev_pos
	}

	fn string(&mut self) -> Option<String> {
		let mut tok = Vec::<u8>::new();
		loop {
			if self.eof() {
				break
			}

			match self.ch() {
				b' '	=> break,
				b'\t'	=> break,
				b'\r'	=> break,
				b'\n'	=> break,
				b'"'	=> break,
				_		=> ()
			}
			
			tok.push(self.ch());
			self.pos += 1;
		}

		match tok.len() {
			0	=> None,
			_	=> Some(String::from_utf8(tok).unwrap())
		}
	}

	fn quoted_string(&mut self) -> Option<String> {
		let mut tok = Vec::<u8>::new();
		self.pos += 1;

		loop {
			if self.eof() {
				break;
			}

			match self.ch() {
				b'"' 	=> { self.pos += 1; break; },
				_ 		=> ()
			}	

			tok.push(self.ch());
			self.pos += 1;			
		}

		match tok.len() {
			0	=> None,
			_	=> Some(String::from_utf8(tok).unwrap())
		}
	}

	fn valid_ws(&self) -> bool {
		if self.eof() {
			return false;
		}

		match self.ch() {
			b' ' 	=> true,
			b'\t' 	=> true,
			b'\r'	=> true,
			b'\n'	=> true,
			_		=> false
		}
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = String;

	fn next(&mut self) -> Option<String> {
		self.skip_ws();
		if self.eof() {
			return None;
		}

		match self.ch() {
			b'"' 	=> self.quoted_string(),
			_		=> self.string()
		}
	}
}

#[test]
fn tokenize() {
	let src = b"   foo   bar \"a b\"  baz   ";
	let lexer = Lexer::new(src);
	let tokens: Vec<_> = lexer.collect();
	assert_eq!(4, tokens.len());
	assert_eq!("foo".to_string(), tokens[0]);
	assert_eq!("bar".to_string(), tokens[1]);
	assert_eq!("a b".to_string(), tokens[2]);
	assert_eq!("baz".to_string(), tokens[3]);
}