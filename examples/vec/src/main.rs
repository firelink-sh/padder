use padder::{Alignment, Source};

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Data<'a> {
    inner: &'a [u8],
}

fn main() {
    let v: Vec<Data> = Vec::from(&[
        Data::default(),
        Data {
            inner: &[0u8, 1, 2, 3],
        },
        Data {
            inner: &[4u8, 5, 6, 7, 8],
        },
    ]);

    let padded = v.pad(5, Alignment::Left, Data::default());
    assert_eq!(
        Vec::from(&[
            Data::default(),
            Data {
                inner: &[0u8, 1, 2, 3]
            },
            Data {
                inner: &[4u8, 5, 6, 7, 8]
            },
            Data::default(),
            Data::default(),
        ]),
        padded,
    );
    println!("(left)   padded: {padded:?}");

    let mut width: usize = 8;
    let v: Vec<char> = Vec::from(&['k', 'a', 'f', 'f', 'e']);
    let mut buf: Vec<char> = Vec::with_capacity(width);
    v.pad_to_buffer(8, Alignment::Right, '!', &mut buf);
    assert_eq!(String::from("!!!kaffe"), buf.iter().collect::<String>(),);
    println!("(right)  padded: '{}'", buf.iter().collect::<String>());

    width = 1;
    let v: Vec<&str> = Vec::from(&["mumin", "troll"]);
    let truncated = v.pad(width, Alignment::Center, " ");
    assert_eq!(Vec::from(&["mumin"]), truncated);
    println!(
        "(center) truncated: '{}'",
        truncated.into_iter().collect::<String>()
    );
}
