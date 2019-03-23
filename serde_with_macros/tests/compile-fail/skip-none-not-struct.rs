extern crate serde_with_macros;

use serde_with_macros::skip_serializing_none;

#[skip_serializing_none]
//~^ error: The attribute can only be applied to struct or enum definitions.
fn test() {}
