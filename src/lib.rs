//! This is the module-level documentation.
//!
//! # Examples
//!
//! ```
//! assert_eq(2 + 2, 4);
//! ```

/// This is the struct-level documentation.
pub struct HelloWorld {
	/// This is the field-level documentation.
	pub message: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
