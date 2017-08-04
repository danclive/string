//! A UTF-8 encoded, growable string.
//!
//! The `Str` type is string type that has owership over the [char]. 
//!
//! # Example
//!
//! You can create a `Str` from a literal string with `Str::from`:
//!
//! ```
//! use str::Str;
//!
//! let hello = Str::from("hello, world!");
//! ```
//! 
//! You can append a [`char`] to a `Str` with the [`push`] method, and
//! append a [`&str`] with the [`push_str`] method;
//!
//! ```
//! use str::Str;
//!
//! let mut hello = Str::from("Hello, ");
//!
//! hello.push('w');
//! hello.push_str("orld!");
//! ```
//!
//! [`&str`]: https://doc.rust-lang.org/std/primitive.str.html
//! [`char`]: https://doc.rust-lang.org/std/primitive.char.html
//! [`push`]: #method.push
//! [`push_str`]: #method.push_str
//!
//! If you have a [`String`], you can create a `Str` from it with the
//! [`from`] method, and you can convert `Str` to [`String`] whit the
//! [`into`] method:
//!
//! ```
//! use str::Str;
//!
//! let hello = String::from("Hello world!");
//!
//! let world = Str::from(hello);
//!
//! let hello_world: String = world.into();
//! ```
//!
//! [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
//! [`from`]: #method.from
//! [`into`]: #method.into

use std::ops;

/// A UTF-8 encoded, growable string.
///
/// The `Str` type is string type that has owership over the [char]. 
///
/// # Example
///
/// You can create a `Str` from a literal string with `Str::from`:
///
/// ```
/// use str::Str;
///
/// let hello = Str::from("hello, world!");
/// ```
/// 
/// You can append a [`char`] to a `Str` with the [`push`] method, and
/// append a [`&str`] with the [`push_str`] method;
///
/// ```
/// use str::Str;
///
/// let mut hello = Str::from("Hello, ");
///
/// hello.push('w');
/// hello.push_str("orld!");
/// ```
///
/// [`&str`]: https://doc.rust-lang.org/std/primitive.str.html
/// [`char`]: https://doc.rust-lang.org/std/primitive.char.html
/// [`push`]: #method.push
/// [`push_str`]: #method.push_str
///
/// If you have a [`String`], you can create a `Str` from it with the
/// [`from`] method, and you can convert `Str` to [`String`] whit the
/// [`into`] method:
///
/// ```
/// use str::Str;
///
/// let hello = String::from("Hello world!");
///
/// let world = Str::from(hello);
///
/// let hello_world: String = world.into();
/// ```
///
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`from`]: #method.from
/// [`into`]: #method.into
///
///
/// # Representation
///
/// A `Str` is made up of three components: a pointer to some chars, a
/// length, and a capacity. The pointer points to an internal buffer `Str`
/// uses to store its data. The length is the munber of bytes currently
/// stored in the buffer, and the capacity is the size of the buffer in
/// chars. As such, the length will always be less than or equal to the
/// capacity.
/// 

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

    pub fn as_ptr(&self) -> *const char {
        self.inner.as_ptr()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let s: String = self.clone().into();
        s.into_bytes()
    }

    pub fn as_slice(&self) -> &[char] {
        self.inner.as_slice()
    }

    pub fn as_slice_mut(&mut self) -> &mut [char] {
        self.inner.as_mut_slice()
    }

    pub fn as_vec(self) -> Vec<char> {
        self.inner
    }

    pub fn retain<F>(&mut self, f: F)
        where F: FnMut(&char) -> bool
    {
        self.inner.retain(f)
    }

    pub fn get(&self, idx: usize) -> Option<&char> {
        self.inner.get(idx)
    }
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut char> {
        self.inner.get_mut(idx)
    }

    pub fn truncate(&mut self, new_len: usize) {
        self.inner.truncate(new_len);
    }

    pub fn push(&mut self, ch: char) {
        self.inner.push(ch);
    }

    pub fn push_str(&mut self, string: &str) {
        self.inner.extend(string.chars())
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

    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner)
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

impl From<Vec<char>> for Str {
    fn from(s: Vec<char>) -> Str {
        Str {
            inner: s
        }
    }
}

impl<'a> From<&'a [char]> for Str {
    fn from(s: &'a [char]) -> Str {
        Str {
            inner: s.to_vec()
        }
    }
}

impl<'a> From<&'a mut [char]> for Str {
    fn from(s: &'a mut [char]) -> Str {
        Str {
            inner: s.to_vec()
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

impl AsRef<Str> for Str {
    fn as_ref(&self) -> &Str {
        self
    }
}

impl AsMut<Str> for Str {
    fn as_mut(&mut self) -> &mut Str {
        self
    }
}

impl AsRef<[char]> for Str {
    fn as_ref(&self) -> &[char] {
        &self.inner
    }
}

impl AsMut<[char]> for Str {
    fn as_mut(&mut self) -> &mut [char] {
        &mut self.inner
    }
}

impl ops::Add for Str {
    type Output = Str;
    fn add(self, other: Str) -> Str {
        let mut self2 = self;
        let mut other = other;
        self2.inner.append(&mut other.inner);
        self2
    }
}

impl ops::AddAssign for Str {
    fn add_assign(&mut self, other: Str) {
        let mut other = other;
        self.inner.append(other.inner.as_mut())
    }
}

impl PartialEq for Str {
    fn eq(&self, other: &Str) -> bool {
        self.inner == other.inner
    }
}

impl PartialOrd for Str {
    fn partial_cmp(&self, other: &Str) -> Option<::std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.inner, &other.inner)
    }
}

impl ops::Index<usize> for Str {
    type Output = char;

    fn index(&self, idx: usize) -> &char {
        &self.inner[idx]
    }
}

impl ops::Index<ops::RangeFrom<usize>> for Str {
    type Output = [char];

    fn index(&self, range: ops::RangeFrom<usize>) -> &[char] {
        self.inner.index(range)
    }
}

impl ops::Index<ops::RangeTo<usize>> for Str {
    type Output = [char];

    fn index(&self, range: ops::RangeTo<usize>) -> &[char] {
        self.inner.index(range)
    }
}

impl ops::Index<ops::RangeFull> for Str {
    type Output = [char];

    fn index(&self, _range: ops::RangeFull) -> &[char] {
        self.as_ref()
    }
}

impl ops::IndexMut<usize> for Str {
    fn index_mut(&mut self, idx: usize) -> &mut char {
        &mut self.inner[idx]
    }
}

impl ops::IndexMut<ops::RangeFrom<usize>> for Str {
    fn index_mut(&mut self, range: ops::RangeFrom<usize>) -> &mut [char] {
        self.inner.index_mut(range)
    }
}

impl ops::IndexMut<ops::RangeTo<usize>> for Str {
    fn index_mut(&mut self, range: ops::RangeTo<usize>) -> &mut [char] {
        self.inner.index_mut(range)
    }
}

impl ops::IndexMut<ops::RangeFull> for Str {
    fn index_mut(&mut self, range: ops::RangeFull) -> &mut [char] {
        self.inner.index_mut(range)
    }
}
