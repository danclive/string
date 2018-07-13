//! A UTF-8 encoded, growable string.
//!
//! The `String2` type is string type that has owership over the [char]. 
//!
//! # Example
//!
//! You can create a `String2` from a literal string with `String2::from`:
//!
//! ```
//! use string2::String2;
//!
//! let hello = String2::from("hello, world!");
//! ```
//! 
//! You can append a [`char`] to a `String2` with the [`push`] method, and
//! append a [`&str`] with the [`push_str`] method;
//!
//! ```
//! use string2::String2;
//!
//! let mut hello = String2::from("Hello, ");
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
//! If you have a [`String`], you can create a `String2` from it with the
//! [`from`] method, and you can convert `String2` to [`String`] whit the
//! [`into`] method:
//!
//! ```
//! use string2::String2;
//!
//! let hello = String::from("Hello world!");
//!
//! let world = String2::from(hello);
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
/// The `String2` type is string type that has owership over the [char]. 
///
/// # Example
///
/// You can create a `String2` from a literal string with `String2::from`:
///
/// ```
/// use string2::String2;
///
/// let hello = String2::from("hello, world!");
/// ```
/// 
/// You can append a [`char`] to a `String2` with the [`push`] method, and
/// append a [`&str`] with the [`push_str`] method;
///
/// ```
/// use string2::String2;
///
/// let mut hello = String2::from("Hello, ");
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
/// If you have a [`String`], you can create a `String2` from it with the
/// [`from`] method, and you can convert `String2` to [`String`] whit the
/// [`into`] method:
///
/// ```
/// use string2::String2;
///
/// let hello = String::from("Hello world!");
///
/// let world = String2::from(hello);
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
/// A `String2` is made up of three components: a pointer to some chars, a
/// length, and a capacity. The pointer points to an internal buffer `String2`
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
/// use string2::String2;
///
/// let story = String2::from("Once upon a time...");
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
/// // We can re-build a String2 out of ptr, len, and capacity. This is all
/// // unsafe because we are responsible for making sure the components
/// // valid:
/// let s = unsafe { String2::from_raw_parts(ptr as *mut _, len, capacity) };
///
/// assert_eq!(String2::from("Once upon a time..."), s);
/// ```
///
/// [`as_ptr`]: #method.as_ptr
/// [`len`]: #method.len
/// [`capacity`]: #method.capacity
///
/// If a `String2` has enough capacity, adding elements to it will not
/// re-allocate. For example, consider this program:
///
/// ```
/// use string2::String2;
///
/// let mut s = String2::new();
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
/// use string2::String2;
///
/// let mut s = String2::with_capacity(25);
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
pub struct String2 {
    inner: Vec<char>
}

impl String2 {
    /// Creates a new empty `String2`.
    ///
    /// Given that the `String2` is empty, this will not allocate any initial
    /// buffer. While that means that this initial operation is very
    /// inexpensive, but may cause excessive allocation later, when you add
    /// data. If you have an idea of how much data the `String2` will hold,
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
    /// use string2::String2;
    ///
    /// let s = String2::new();
    /// ```
    #[inline]
    pub fn new() -> String2 {
        String2 {
            inner: Vec::new()
        }
    }

    /// Creates a new empty `String2` with a particular capacity.
    ///
    /// `String2`s have an internal buffer to hold their data. The capacity is
    /// the length of that buffer, and can be queried with the [`capacity`]
    /// method. This method creates an empty `String2`, but one with an initial
    /// buffer that can hold `capacity` bytes. This is useful when you may be
    /// appending a bunch of data to the `String2`, reducing the number of
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
    /// use string2::String2;
    ///
    /// let mut s = String2::with_capacity(10);
    ///
    /// // The String2 contains no chars, even though it has capacity for more
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
    #[inline]
    pub fn with_capacity(capacity: usize) -> String2 {
        String2 {
            inner: Vec::with_capacity(capacity)
        }
    }

    /// Returns this `String2`'s capacity, in bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let s = String2::with_capacity(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Ensures that this `String2`'s capacity is at least `additional` bytes
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
    /// use string2::String2;
    ///
    /// let mut s = String2::new();
    ///
    /// s.reserve(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    ///
    /// This may not actually increase the capacity:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let mut s = String2::with_capacity(10);
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
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Ensures that this `String2`'s capacity is `additional` bytes
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
    /// use string2::String2;
    ///
    /// let mut s = String2::new();
    ///
    /// s.reserve_exact(10);
    ///
    /// assert!(s.capacity() >= 10);
    /// ```
    ///
    /// This may not actually increase the capacity:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let mut s = String2::with_capacity(10);
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
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    /// Shrinks the capacity of this `String2` to match its length.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let mut s = String2::from("foo");
    ///
    /// s.reserve(100);
    /// assert!(s.capacity() >= 100);
    ///
    /// s.shrink_to_fit();
    /// assert_eq!(3, s.capacity());
    /// ```
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Converts a `String2` to a raw pointer.
    /// As `String2` are a vector of chars, the raw pointer points to a char.
    /// This pointer will be pointing to the first byte of the `String2`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let s = String2::from("Hello");
    /// let ptr = s.as_ptr();
    /// ```
    #[inline]
    pub fn as_ptr(&self) -> *const char {
        self.inner.as_ptr()
    }

    /// Creates a new `String2` from a length, capacity, and pointer.
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
    /// `String2` which may then deallocate, reallocate or change the
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
    /// use string2::String2;
    ///
    /// let s = String2::from("hello");
    /// let ptr = s.as_ptr();
    /// let len = s.len();
    /// let capacity = s.capacity();
    ///
    /// mem::forget(s);
    ///
    /// let s = unsafe { String2::from_raw_parts(ptr as *mut _, len, capacity) };
    ///
    /// assert_eq!(String2::from("hello"), s);
    /// ```
    #[inline]
    pub unsafe fn from_raw_parts(buf: *mut char, length: usize, capacity: usize) -> String2 {
        String2 {
            inner: Vec::from_raw_parts(buf, length, capacity)
        }
    }

    /// Converts a `String2` into a byte vector.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let s = String2::from("hello");
    /// let bytes = s.as_bytes();
    ///
    /// assert_eq!(&[104, 101, 108, 108, 111], &bytes[..]);
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> Vec<u8> {
        let s: String = self.clone().into();
        s.into_bytes()
    }

    /// Converts a `String2` into a char slice.
    ///
    /// This consumes the `String2`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let s = String2::from("hello");
    /// let bytes = s.as_slice();
    ///
    /// assert_eq!(&['h', 'e', 'l', 'l', 'o'][..], &bytes[..]);
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &[char] {
        self.inner.as_slice()
    }

    /// Converts a `String2` into a mut char slice.
    ///
    /// This consumes the `String2`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let mut s = String2::from("hello");
    /// {
    ///     let bytes = s.as_mut_slice();
    ///     bytes[1] = 'a';
    /// }
    ///
    /// assert_eq!(String2::from("hallo"), s);
    /// ```
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [char] {
        self.inner.as_mut_slice()
    }

    /// Converts a `String2` into a char vector.
    ///
    /// This consumes the `String2`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let s = String2::from("hello");
    /// let bytes = s.as_vec();
    ///
    /// assert_eq!(&['h', 'e', 'l', 'l', 'o'], &bytes[..]);
    /// ```
    #[inline]
    pub fn as_vec(self) -> Vec<char> {
        self.inner
    }

    /// Converts a `String2` into a mut char slice.
    ///
    /// This consumes the `String2`, so we do not need to copy its contents.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use string2::String2;
    ///
    /// let mut s = String2::from("hello");
    /// {
    ///     let bytes = s.as_mut_vec();
    ///     bytes[1] = 'a';
    /// }
    ///
    /// assert_eq!(String2::from("hallo"), s);
    /// ```
    #[inline]
    pub fn as_mut_vec(&mut self) -> &mut Vec<char> {
        &mut self.inner
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
        where F: FnMut(&char) -> bool
    {
        self.inner.retain(f)
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Option<&char> {
        self.inner.get(idx)
    }

    #[inline]
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut char> {
        self.inner.get_mut(idx)
    }

    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        self.inner.truncate(new_len);
    }

    #[inline]
    pub fn push(&mut self, ch: char) {
        self.inner.push(ch);
    }

    #[inline]
    pub fn push_str(&mut self, string: &str) {
        self.inner.extend(string.chars())
    }

    #[inline]
    pub fn pop(&mut self) -> Option<char> {
        self.inner.pop()
    }

    #[inline]
    pub fn remove(&mut self, idx: usize) -> char {
        self.inner.remove(idx)
    }

    #[inline]
    pub fn insert(&mut self, idx: usize, ch: char) {
        self.inner.insert(idx, ch);
    }

    #[inline]
    pub fn insert_str(&mut self, _idx: usize, _string: &str) {
        
    }

    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn split_off(&mut self, at: usize) -> String2 {
        let other = self.inner.split_off(at);

        String2 {
            inner: other
        }
    }

    #[inline]
    pub fn split_at(&self, mid: usize) -> (String2, String2) {
        let (a, b) = self.inner.split_at(mid);

        (String2 { inner: a.to_vec() }, String2 { inner: b.to_vec() })
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    #[inline]
    pub fn iter(self) -> StrIterator {
        self.into_iter()
    }
}

impl<'a> From<&'a str> for String2 {
    #[inline]
    fn from(string: &'a str) -> String2 {
        String2 {
            inner: string.chars().collect()
        }
    }
}

impl From<String> for String2 {
    #[inline]
    fn from(string: String) -> String2 {
        String2 {
            inner: string.chars().collect()
        }
    }
}

impl From<Vec<char>> for String2 {
    #[inline]
    fn from(s: Vec<char>) -> String2 {
        String2 {
            inner: s
        }
    }
}

impl<'a> From<&'a [char]> for String2 {
    #[inline]
    fn from(s: &'a [char]) -> String2 {
        String2 {
            inner: s.to_vec()
        }
    }
}

impl<'a> From<&'a mut [char]> for String2 {
    #[inline]
    fn from(s: &'a mut [char]) -> String2 {
        String2 {
            inner: s.to_vec()
        }
    }
}

impl Into<String> for String2 {
    fn into(self) -> String {
        self.inner.iter().map(|c| c.encode_utf8(&mut [0; 4]).to_string()).collect()
    }
}

impl<'a> Into<String> for &'a String2 {
    fn into(self) -> String {
        self.inner.iter().map(|c| c.encode_utf8(&mut [0; 4]).to_string()).collect()
    }
}

impl Default for String2 {
    #[inline]
    fn default() -> String2 {
        String2::new()
    }
}

impl IntoIterator for String2 {
    type Item = char;
    type IntoIter = StrIterator;
    #[inline]
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
    #[inline]
    fn next(&mut self) -> Option<char> {
        self.inner.next()
    }
}

impl AsRef<String2> for String2 {
    #[inline]
    fn as_ref(&self) -> &String2 {
        self
    }
}

impl AsMut<String2> for String2 {
    #[inline]
    fn as_mut(&mut self) -> &mut String2 {
        self
    }
}

impl AsRef<[char]> for String2 {
    #[inline]
    fn as_ref(&self) -> &[char] {
        &self.inner
    }
}

impl AsMut<[char]> for String2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [char] {
        &mut self.inner
    }
}

impl ops::Add for String2 {
    type Output = String2;
    #[inline]
    fn add(self, other: String2) -> String2 {
        let mut self2 = self;
        let mut other = other;
        self2.inner.append(&mut other.inner);
        self2
    }
}

impl ops::Add<char> for String2 {
    type Output = String2;
    #[inline]
    fn add(mut self, other: char) -> String2 {
        self.push(other);
        self
    }
}

impl<'a> ops::Add<&'a str> for String2 {
    type Output = String2;
    #[inline]
    fn add(mut self, other: &str) -> String2 {
        self.push_str(other);
        self
    }
}

impl ops::AddAssign for String2 {
    #[inline]
    fn add_assign(&mut self, other: String2) {
        let mut other = other;
        self.inner.append(other.inner.as_mut())
    }
}

impl ops::AddAssign<char> for String2 {
    #[inline]
    fn add_assign(&mut self, other: char) {
        self.push(other)
    }
}

impl<'a> ops::AddAssign<&'a str> for String2 {
    #[inline]
    fn add_assign(&mut self, other: &str) {
        self.push_str(other)
    }
}

impl PartialEq for String2 {
    #[inline]
    fn eq(&self, other: &String2) -> bool {
        self.inner == other.inner
    }
}

impl PartialOrd for String2 {
    #[inline]
    fn partial_cmp(&self, other: &String2) -> Option<::std::cmp::Ordering> {
        PartialOrd::partial_cmp(&self.inner, &other.inner)
    }
}

impl ops::Index<usize> for String2 {
    type Output = char;
    #[inline]
    fn index(&self, idx: usize) -> &char {
        &self.inner[idx]
    }
}

impl ops::Index<ops::Range<usize>> for String2 {
    type Output = [char];
    #[inline]
    fn index(&self, range: ops::Range<usize>) -> &[char] {
        self.inner.index(range)
    }
}

impl ops::Index<ops::RangeFrom<usize>> for String2 {
    type Output = [char];
    #[inline]
    fn index(&self, range: ops::RangeFrom<usize>) -> &[char] {
        self.inner.index(range)
    }
}

impl ops::Index<ops::RangeTo<usize>> for String2 {
    type Output = [char];
    #[inline]
    fn index(&self, range: ops::RangeTo<usize>) -> &[char] {
        self.inner.index(range)
    }
}

impl ops::Index<ops::RangeFull> for String2 {
    type Output = [char];
    #[inline]
    fn index(&self, _range: ops::RangeFull) -> &[char] {
        self.as_ref()
    }
}

impl ops::IndexMut<usize> for String2 {
    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut char {
        &mut self.inner[idx]
    }
}

impl ops::IndexMut<ops::Range<usize>> for String2 {
    #[inline]
    fn index_mut(&mut self, range: ops::Range<usize>) -> &mut [char] {
        self.inner.index_mut(range)
    }
}

impl ops::IndexMut<ops::RangeFrom<usize>> for String2 {
    #[inline]
    fn index_mut(&mut self, range: ops::RangeFrom<usize>) -> &mut [char] {
        self.inner.index_mut(range)
    }
}

impl ops::IndexMut<ops::RangeTo<usize>> for String2 {
    #[inline]
    fn index_mut(&mut self, range: ops::RangeTo<usize>) -> &mut [char] {
        self.inner.index_mut(range)
    }
}

impl ops::IndexMut<ops::RangeFull> for String2 {
    #[inline]
    fn index_mut(&mut self, range: ops::RangeFull) -> &mut [char] {
        self.inner.index_mut(range)
    }
}

impl fmt::Display for String2 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.into();
        fmt::Display::fmt(&s, f)
    }
}

impl fmt::Debug for String2 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.into();
        fmt::Debug::fmt(&s, f)
    }
}
