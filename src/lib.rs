pub use paste::paste;

/// Provides a macro to generate a function converting byte slices to vectors of a specified type.
///
/// The `bytes_to_type!` macro generates a conversion function that transforms a byte slice (`&[u8]`)
/// into a `Vec<T>`, where `T` is a type specified as an argument to the macro. The resulting function
/// returns a `Result<Vec<T>, anyhow::Error>` to gracefully handle potential conversion failures.
///
/// # Example
///
/// ```
/// use bytes_to_type::bytes_to_type;
///
/// // This macro generates a function with the signature:
/// // pub fn bytes_to_u32(bytes: &[u8]) -> Result<Vec<u32>, anyhow::Error>
/// bytes_to_type!(u32);
///
/// let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8];
/// let result = bytes_to_u32(&bytes).expect("Failed to convert bytes to u32");
///
/// assert_eq!(result, vec![67305985, 134678021]);
/// ```
///
/// # Generated Function
///
/// The generated function ensures the safe conversion of byte slices into `Vec<T>` while preserving
/// the byte order. If the length of the byte slice is not a multiple of the size of `T` or if the
/// conversion fails due to other reasons, the function returns an `Err` variant containing an error
/// message.
///
/// # Usage Note
///
/// Be mindful of the byte order and potential alignment issues during conversions to prevent unintended
/// results or panics during runtime.
///

#[macro_export]
macro_rules! bytes_to_type {
    ($type:ty) => {
        $crate::paste! {
            pub fn [<bytes_to_$type>](bytes: &[u8]) -> anyhow::Result<Vec<$type>> {
                if bytes.len() % std::mem::size_of::<$type>() != 0 {
                    return Err(anyhow::anyhow!(
                        "Bytes length is not a multiple of {}",
                        std::mem::size_of::<$type>()
                    ));
                }

                Ok(unsafe {
                    std::slice::from_raw_parts(
                        bytes.as_ptr() as *const $type,
                        bytes.len() / std::mem::size_of::<$type>(),
                    )
                }
                .to_vec())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        bytes_to_type!(u32);

        let bytes = vec![1, 2, 3, 4];
        let result = bytes_to_u32(bytes.as_slice()).unwrap();

        assert_eq!(result, vec![67305985]);
    }

    #[test]
    fn it_returns_error_if_bytes_length_is_not_a_multiple_of_type_size() {
        bytes_to_type!(u32);

        let bytes = vec![1, 2, 3];
        let result = bytes_to_u32(bytes.as_slice());

        assert!(result.is_err());
    }
}
