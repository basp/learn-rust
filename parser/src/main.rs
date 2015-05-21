struct Lexer<'a> {
	tok: Vec<u8>,
	buf: &'a [u8],
	pos: usize
}

impl<'a> Lexer<'a> {
	fn new(buf: &[u8]) -> Lexer {
		Lexer {
			tok: Vec::<u8>::new(),
			buf: buf,
			pos: 0
		}
	}
}

impl<'a> Lexer<'a> {
	fn next(&mut self) -> Option<Vec<u8>> {
		loop {
			if self.pos >= self.buf.len() {
				return None;
			}

			println!("{:?}", self.buf[self.pos]);

			match self.buf[self.pos] {
				b'"' => return None,
				b' ' => break,
				_ => self.tok.push(self.buf[self.pos])
			}

			self.pos += 1;
		}
		Some(self.tok.clone())
	}

	fn skip_ws(&mut self) {
		loop {
			if self.pos >= self.buf.len() {
				break;
			}

			match self.buf[self.pos] {
				b' ' => (),
				b'\t' => (),
				_ => break
			}

			self.pos += 1;
		}
	}
}

fn main() {
	let buf = b"foo bar quux";
	let mut lexer = Lexer::new(buf);
	let tok = lexer.next();
	println!("{:?}", tok);
}
