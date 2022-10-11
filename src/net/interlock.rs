use heapless::String;
use miniconf::{Error, Miniconf, MiniconfMetadata};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Interlock {
    /// Set the interlock to armed (true) or disarmed (false).
    ///
    /// # Value
    /// True to arm, false to disarm.
    pub armed: bool,

    /// Interlock target.
    /// The Interlock will publish its state (true or false) onto this mqtt path.
    /// Full path to the desired target. No wildcards.
    ///
    /// # Value
    /// Any string up to 128 characters.
    pub target: String<128>,
}

impl Miniconf for Interlock {
    fn string_set(
        &mut self,
        mut topic_parts: core::iter::Peekable<core::str::Split<char>>,
        value: &[u8],
    ) -> Result<(), Error> {
        if topic_parts.peek().is_some() {
            return Err(Error::PathTooLong);
        }
        *self = serde_json_core::from_slice(value)?.0;
        Ok(())
    }

    fn string_get(
        &self,
        mut topic_parts: core::iter::Peekable<core::str::Split<char>>,
        value: &mut [u8],
    ) -> Result<usize, Error> {
        if topic_parts.peek().is_some() {
            return Err(Error::PathTooLong);
        }

        serde_json_core::to_slice(self, value).map_err(|_| Error::SerializationFailed)
    }

    fn get_metadata(&self) -> MiniconfMetadata {
        MiniconfMetadata {
            // No topic length is needed, as there are no sub-members.
            max_topic_size: 0,
            // One index is required for the current element.
            max_depth: 1,
        }
    }

    // This implementation is the base case for primitives where it will
    // yield once for self, then return None on subsequent calls.
    fn recurse_paths<const TS: usize>(
        &self,
        index: &mut [usize],
        _topic: &mut heapless::String<TS>,
    ) -> Option<()> {
        if index.is_empty() {
            // Note: During expected execution paths using `iter()`, the size of the
            // index stack is checked in advance to make sure this condition doesn't occur.
            // However, it's possible to happen if the user manually calls `recurse_paths`.
            unreachable!("Index stack too small");
        }

        let i = index[0];
        index[0] += 1;
        index[1..].iter_mut().for_each(|x| *x = 0);

        if i == 0 {
            Some(())
        } else {
            None
        }
    }
}
