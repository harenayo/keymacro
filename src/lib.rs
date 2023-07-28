//! Keyword-like macros for Rust.

#![no_std]

/// Keeps the value alive.
///
/// # Examples
///
/// ```
/// use {
///     keymacro::keep,
///     std::rc::Rc,
/// };
///
/// let rc = Rc::new(());
/// assert_eq!(Rc::strong_count(&rc), 1);
///
/// // Clone rc and drop it.
/// Rc::clone(&rc);
/// assert_eq!(Rc::strong_count(&rc), 1);
///
/// // Clone rc and keep it.
/// keep!(Rc::clone(&rc));
/// assert_eq!(Rc::strong_count(&rc), 2);
/// ```
#[macro_export]
macro_rules! keep {
    ($value:expr) => {
        let _keeped = $value;
    };
}

/// A RAII implementation of `defer`.
///
/// # Examples
///
/// ```
/// use {
///     keymacro::Defer,
///     std::cell::Cell,
/// };
///
/// let changed = Cell::new(false);
///
/// {
///     let _defer = Defer::new(|| Cell::set(&changed, true));
///     assert!(!Cell::get(&changed));
/// }
///
/// assert!(Cell::get(&changed));
/// ```
#[must_use]
pub struct Defer<F: FnOnce()> {
    deferred: Option<F>,
}

impl<F: FnOnce()> Defer<F> {
    /// Creates a new instance of `defer`.
    pub const fn new(deferred: F) -> Self {
        Self {
            deferred: Option::Some(deferred),
        }
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Option::Some(deferred) = self.deferred.take() {
            deferred();
        }
    }
}

/// A keyword-like macro to create [`Defer`].
///
/// # Examples
///
/// ```
/// use {
///     keymacro::deferred,
///     std::cell::Cell,
/// };
///
/// let changed = Cell::new(false);
///
/// {
///     let _defer = deferred! {
///         Cell::set(&changed, true);
///     };
///
///     assert!(!Cell::get(&changed));
/// }
///
/// assert!(Cell::get(&changed));
/// ```
#[macro_export]
macro_rules! deferred {
    ($($token:tt)*) => {
        $crate::Defer::new(|| { $($token)* })
    };
}

/// A keyword-like macro to create [`Defer`].
///
/// # Examples
///
/// ```
/// use {
///     keymacro::defer,
///     std::cell::Cell,
/// };
///
/// let changed = Cell::new(false);
///
/// {
///     defer! {
///         Cell::set(&changed, true);
///     }
///
///     assert!(!Cell::get(&changed));
/// }
///
/// assert!(Cell::get(&changed));
/// ```
#[macro_export]
macro_rules! defer {
    ($($token:tt)*) => {
        $crate::keep!($crate::deferred!($($token)*));
    };
}

/// A macro to write text.
///
/// # Examples
///
/// ```
/// use keymacro::text;
///
/// assert_eq!(
///     text!(
///         "This is the first line."
///         "You can write more lines."
///     ),
///     "This is the first line.\nYou can write more lines."
/// );
/// ```
#[macro_export]
macro_rules! text {
    () => {
        ""
    };
    ($first:literal $($more:literal)*) => {
        concat!($first $(, '\n', $more)*)
    };
}
