
#[derive(Clone, Debug)]
pub struct Str {
    inner: Vec<char>
}

impl Str {
	pub fn new() -> Str {
		Str {
			inner: Vec::new()
		}
	}

	pub fn with_capacity(capacity: usize) -> Str {
		Str {
			inner: Vec::with_capacity(capacity)
		}
	}

	pub fn capacity(&self) -> usize {
		self.inner.capacity()
	}

	pub fn reserve(&mut self, additional: usize) {
		self.inner.reserve(additional);
	}

	pub fn reserve_exact(&mut self, additional: usize) {
		self.inner.reserve_exact(additional);
	}

	pub fn shrink_to_fit(&mut self) {
		self.inner.shrink_to_fit();
	}

	pub fn push(&mut self, ch: char) {
		self.inner.push(ch);
	}

	/*

	pub fn as_bytes(&self) -> &[u8] {

		let a: String = self.clone().into();

		a.as_bytes()
	}
	*/

	// as_ptr(&self) -> *const u8

	// get
	// get_mut

	pub fn truncate(&mut self, new_len: usize) {
		self.inner.truncate(new_len);
	}

	pub fn pop(&mut self) -> Option<char> {
		self.inner.pop()
	}

	pub fn remove(&mut self, idx: usize) -> char {
		self.inner.remove(idx)
	}

	pub fn insert(&mut self, idx: usize, ch: char) {
		self.inner.insert(idx, ch);
	}

	pub fn insert_str(&mut self, _idx: usize, _string: &str) {

	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}

	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}

	pub fn split_off(&mut self, at: usize) -> Str {
		let other = self.inner.split_off(at);

		Str {
			inner: other
		}
	}

	pub fn split_at(&self, mid: usize) -> (Str, Str) {
		let (a, b) = self.inner.split_at(mid);

		(Str { inner: a.to_vec() }, Str { inner: b.to_vec() })
	}

	/*
	pub fn from_mut_char<'a>(char: &'a mut [char]) -> &'a mut Str {
		let mut a = Str {
			inner: char.to_vec()
		};

		a.as_mut()
	}
	*/

	/*
	pub fn split_at_mut(&mut self, mid: usize) -> (&mut Str, &mut Str) {
	 	let (a, b) = self.inner.split_at_mut(mid);

	 	(Str::from_mut_char(a), Str::from_mut_char(b))
	}
	*/

	pub fn clear(&mut self) {
		self.inner.clear()
	}

	pub fn iter(self) -> StrIterator {
		self.into_iter()
	}
}

impl<'a> From<&'a str> for Str {
	fn from(string: &'a str) -> Str {
		Str {
			inner: string.chars().collect()
		}
	}
}

impl From<String> for Str {
	fn from(string: String) -> Str {
		Str {
			inner: string.chars().collect()
		}
	}
}

impl Into<String> for Str {
	fn into(self) -> String {
		self.inner.iter().map(|c| c.encode_utf8(&mut [0; 4]).to_string()).collect()
	}
}

impl Default for Str {
	#[inline]
	fn default() -> Str {
		Str::new()
	}
}

impl IntoIterator for Str {
	type Item = char;
	type IntoIter = StrIterator;

	fn into_iter(self) -> Self::IntoIter {
		StrIterator {
			inner: self.inner.into_iter()
		}
	}
}

pub struct StrIterator {
	inner: ::std::vec::IntoIter<char>
}

impl Iterator for StrIterator {
	type Item = char;
	fn next(&mut self) -> Option<char> {
		self.inner.next()
	}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
