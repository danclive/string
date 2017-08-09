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
use std::fmt;

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
/// The buffer is always stored on the heap.
///
/// You can look at these with the [`as_ptr`], [`len`], and [`capacity`]
/// methods:
///
/// ```
/// use std::mem;
/// use str::Str;
///
/// let story = Str::from("Once upon a time...");
///
/// let ptr = story.as_ptr();
/// let len = story.len();
/// let capacity = story.capacity();
///
/// // story has nineteen chars
/// assert_eq!(19, len);
///
/// // Now that we have our parts, we throw the story away.
/// mem::forget(story);
///
/// // We can re-build a Str out of ptr, len, and capacity. This is all
/// // unsafe because we are responsible for making sure the components
/// // valid:
/// let s = unsafe { Str::from_raw_parts(ptr as *mut _, len, capacity) };
///
/// assert_eq!(Str::from("Once upon a time..."), s);
/// ```
///
/// [`as_ptr`]: #method.as_ptr
/// [`len`]: #method.len
/// [`capacity`]: #method.capacity
///
/// If a `Str` has enough capacity, adding elements to it will not
/// re-allocate. For example, consider this program:
///
/// ```
/// use str::Str;
///
/// let mut s = Str::new();
///
/// println!("{}", s.capacity());
///
/// for _ in 0..5 {
///     s.push_str("hello");
///     println!("{}", s.capacity());
/// }
/// ```
///
/// This will output the following:
///
/// ```text
/// 0
/// 5
/// 10
/// 20
/// 20
/// 40
/// ```
///
/// At first, we have no memory allocated at all, but as we append to the
/// string, it increases its capacity appropriately. If we instead use the
/// [`with_capacity`] method to allocate the correct capacity initially:
///
/// ```
/// use str::Str;
///
/// let mut s = Str::with_capacity(25);
///
/// println!("{}", s.capacity());
///
/// for _ in 0..5 {
///     s.push_str("hello");
///     println!("{}", s.capacity());
/// }
/// ```
///
/// [`with_capacity`]: #method.with_capacity
///
/// We end up with a different output:
///
/// ```text
/// 25
/// 25
/// 25
/// 25
/// 25
/// 25
/// ```
///
/// Here, there's no need to allocate more memory inside the loop.
#[derive(Clone, Eq, Ord)]
pub struct Str {
    inner: Vec<char>
}

impl Str {
    /// Creates a new empty `Str`.
    ///
    /// Given that the `Str` is empty, this will not allocate any initial
    /// buffer. While that means that this initial operation is very
    /// inexpensive, but may cause excessive allocation later, when you add
    /// data. If you have an idea of how much data the `Str` will hold,
    /// consider the [`with_capacity`] method to prevent excessive
    /// re-allocation.
    ///
    /// [`with_capacity`]: #method.with_capacity
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let s = Str::new();
    /// ```
    pub fn new() -> Str {
        Str {
            inner: Vec::new()
        }
    }

    /// Creates a new empty `Str` with a particular capacity.
    ///
    /// `Str`s have an internal buffer to hold their data. The capacity is
    /// the length of that buffer, and can be queried with the [`capacity`]
    /// method. This method creates an empty `Str`, but one with an initial
    /// buffer that can hold `capacity` bytes. This is useful when you may be
    /// appending a bunch of data to the `Str`, reducing the number of
    /// reallocations it needs to do.
    ///
    /// [`capacity`]: #method.capacity
    ///
    /// If the given capacity is `0`, no allocation will occur, and this method
    /// is identical to the [`new`] method.
    ///
    /// [`new`]: #method.new
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let mut s = Str::with_capacity(10);
    ///
    /// // The Str contains no chars, even though it has capacity for more
    /// assert_eq!(s.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// let cap = s.capacity();
    /// for i in 0..10 {
    ///     s.push('a');
    /// }
    ///
    /// assert_eq!(s.capacity(), cap);
    ///
    /// // ...but this may make the vector reallocate
    /// s.push('a');
    /// ```
    pub fn with_capacity(capacity: usize) -> Str {
        Str {
            inner: Vec::with_capacity(capacity)
        }
    }

    /// Returns this `Str`'s capacity, in bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let s = Str::with_capacity(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Ensures that this `Str`'s capacity is at least `additional` bytes
    /// larger than its length.
    ///
    /// The capacity may be increased by more than `additional` bytes if it
    /// chooses, to prevent frequent reallocations.
    ///
    /// If you do not want this "at least" behavior, see the [`reserve_exact`]
    /// method.
    ///
    /// [`reserve_exact`]: #method.reserve_exact
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let mut s = Str::new();
    ///
    /// s.reserve(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    ///
    /// This may not actually increase the capacity:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let mut s = Str::with_capacity(10);
    /// s.push('a');
    /// s.push('b');
    ///
    /// // s now has a length of 2 and a capacity of 10
    /// assert_eq!(2, s.len());
    /// assert_eq!(10, s.capacity());
    ///
    /// // Since we already have an extra 8 capacity, calling this...
    /// s.reserve(8);
    ///
    /// // ... doesn't actually increase.
    /// assert_eq!(10, s.capacity());
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Ensures that this `Str`'s capacity is `additional` bytes
    /// larger than its length.
    ///
    /// Consider using the [`reserve`] method unless you absolutely know
    /// better than the allocator.
    ///
    /// [`reserve`]: #method.reserve
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let mut s = Str::new();
    ///
    /// s.reserve_exact(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    ///
    /// This may not actually increase the capacity:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let mut s = Str::with_capacity(10);
    /// s.push('a');
    /// s.push('b');
    ///
    /// // s now has a length of 2 and a capacity of 10
    /// assert_eq!(2, s.len());
    /// assert_eq!(10, s.capacity());
    ///
    /// // Since we already have an extra 8 capacity, calling this...
    /// s.reserve_exact(8);
    ///
    /// // ... doesn't actually increase.
    /// assert_eq!(10, s.capacity());
    /// ```

    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    /// Shrinks the capacity of this `Str` to match its length.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let mut s = Str::from("foo");
    ///
    /// s.reserve(100);
    /// assert!(s.capacity() >= 100);
    ///
    /// s.shrink_to_fit();
    /// assert_eq!(3, s.capacity());
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Converts a `Str` to a raw pointer.
    /// As `Str` are a vector of chars, the raw pointer points to a char.
    /// This pointer will be pointing to the first byte of the `Str`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let s = Str::from("Hello");
    /// let ptr = s.as_ptr();
    /// ```
    pub fn as_ptr(&self) -> *const char {
        self.inner.as_ptr()
    }

    /// Creates a new `Str` from a length, capacity, and pointer.
    ///
    /// # Safety
    ///
    /// This is highly unsafe, due to the number of invariants that aren't
    /// checked:
    ///
    /// * The memory at `ptr` needs to have been previously allocated by the
    ///   same allocator the standard library uses.
    /// * `length` needs to be less than or equal to `capacity`.
    /// * `capacity` needs to be the correct value.
    ///
    /// Violating these may cause problems like corrupting the allocator's
    /// internal datastructures.
    ///
    /// The ownership of `ptr` is effectively transferred to the
    /// `Str` which may then deallocate, reallocate or change the
    /// contents of memory pointed to by the pointer at will. Ensure
    /// that nothing else uses the pointer after calling this
    /// function.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::mem;
    /// use str::Str;
    ///
    /// let s = Str::from("hello");
    /// let ptr = s.as_ptr();
    /// let len = s.len();
    /// let capacity = s.capacity();
    ///
    /// mem::forget(s);
    ///
    /// let s = unsafe { Str::from_raw_parts(ptr as *mut _, len, capacity) };
    ///
    /// assert_eq!(Str::from("hello"), s);
    /// ```
    pub unsafe fn from_raw_parts(buf: *mut char, length: usize, capacity: usize) -> Str {
        Str {
            inner: Vec::from_raw_parts(buf, length, capacity)
        }
    }

    /// Converts a `Str` into a byte vector.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let s = Str::from("hello");
    /// let bytes = s.as_bytes();
    ///
    /// assert_eq!(&[104, 101, 108, 108, 111], &bytes[..]);
    /// ```
    pub fn as_bytes(&self) -> Vec<u8> {
        let s: String = self.clone().into();
        s.into_bytes()
    }

    /// Converts a `Str` into a char slice.
    ///
    /// This consumes the `Str`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let s = Str::from("hello");
    /// let bytes = s.as_slice();
    ///
    /// assert_eq!(&['h', 'e', 'l', 'l', 'o'][..], &bytes[..]);
    /// ```
    pub fn as_slice(&self) -> &[char] {
        self.inner.as_slice()
    }

    /// Converts a `Str` into a mut char slice.
    ///
    /// This consumes the `Str`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let mut s = Str::from("hello");
    /// {
    ///     let bytes = s.as_mut_slice();
    ///     bytes[1] = 'a';
    /// }
    ///
    /// assert_eq!(Str::from("hallo"), s);
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [char] {
        self.inner.as_mut_slice()
    }

    /// Converts a `Str` into a char vector.
    ///
    /// This consumes the `Str`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let s = Str::from("hello");
    /// let bytes = s.as_vec();
    ///
    /// assert_eq!(&['h', 'e', 'l', 'l', 'o'], &bytes[..]);
    /// ```
    pub fn as_vec(self) -> Vec<char> {
        self.inner
    }

    /// Converts a `Str` into a mut char slice.
    ///
    /// This consumes the `Str`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use str::Str;
    ///
    /// let mut s = Str::from("hello");
    /// {
    ///     let bytes = s.as_mut_vec();
    ///     bytes[1] = 'a';
    /// }
    ///
    /// assert_eq!(Str::from("hallo"), s);
    /// ```
    pub fn as_mut_vec(&mut self) -> &mut Vec<char> {
        &mut self.inner
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

impl<'a> Into<String> for &'a Str {
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

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.into();
        fmt::Display::fmt(&s, f)
    }
}

impl fmt::Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.into();
        fmt::Debug::fmt(&s, f)
    }
}
