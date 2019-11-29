// Macros can now be used in type position (RFC 873), and attributes can be applied to statements (RFC 16):
// Use a macro to name a type
macro_rules! Tuple {
    { $A:ty,$B:ty } => { ($A, $B) }
}

fn main() {
    let x: Tuple!(i32, i32) = (1, 2);
}

