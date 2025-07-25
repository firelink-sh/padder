use padder::{Alignment, Source};

fn main() {
    let s1 = String::from("norris");

    // Consume `s` and create a new `String`.
    let padded = s1.pad(10, Alignment::Left, '🚗');
    assert_eq!("norris🚗🚗🚗🚗", padded);
    println!("(left)   padded: '{}'", &padded,);

    let mut width = 10;
    let s2 = String::from("rally");
    let mut buf = String::with_capacity(width);
    s2.pad_to_buffer(width, Alignment::Right, '_', &mut buf);
    assert_eq!("_____rally", buf);
    println!("(right)  padded: '{}'", &buf,);

    width = 2;
    let s3 = String::from("verstappen");
    // Passing a width smaller than the string sice performs a truncate.
    // When truncating - the `symbol` has no effect.
    let truncated = s3.pad(width, Alignment::Center, '_');
    assert_eq!("ta", truncated);
    println!("(center) padded: '{}'", &truncated,);
}
