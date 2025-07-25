use padder::{Alignment, MutableSource};

fn main() {
    let width: usize = 11;
    let original_vec: Vec<char> = "piastri".chars().collect();

    let mut vec_pad_left: Vec<char> = original_vec.clone();
    vec_pad_left.shrink_to_fit();
    (&mut vec_pad_left).pad(width, Alignment::Left, '|'); // "piastri||||"
    println!(
        "(left)   padded: '{}'",
        vec_pad_left.iter().collect::<String>(),
    );

    let mut vec_pad_right: Vec<char> = original_vec.clone();
    vec_pad_right.shrink_to_fit();
    (&mut vec_pad_right).pad(width, Alignment::Right, 'ö'); // "ööööpiastri"
    println!(
        "(right)  padded: '{}'",
        vec_pad_right.iter().collect::<String>(),
    );

    let mut vec_pad_center: Vec<char> = original_vec.clone();
    vec_pad_center.shrink_to_fit();
    (&mut vec_pad_center).pad(width, Alignment::Center, '-'); // "--piastri--"
    println!(
        "(center) padded: '{}'",
        vec_pad_center.iter().collect::<String>(),
    );
}
