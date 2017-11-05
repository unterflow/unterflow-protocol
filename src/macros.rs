macro_rules! size_of {
($($t:ty), *) => {{
    let mut length = 0;
    $(length += ::std::mem::size_of::<$t>();); *
    length as usize
}};
}
