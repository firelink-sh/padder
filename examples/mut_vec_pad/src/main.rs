use padder::{Alignment, MutableSource};

fn main() {
    let width: usize = 20;
    let original_vec: Vec<char> = "plant needs water".chars().collect();

    let mut vec_pad_left: Vec<char> = original_vec.clone();
    vec_pad_left.shrink_to_fit();
    (&mut vec_pad_left).pad(width, Alignment::Left, 'ö');
    println!(
        "(left)   padded string: {}...",
        vec_pad_left
            .iter()
            .collect::<String>()
            .chars()
            .take(10)
            .collect::<String>()
    );

    let mut vec_pad_right: Vec<char> = original_vec.clone();
    vec_pad_right.shrink_to_fit();
    (&mut vec_pad_right).pad(width, Alignment::Right, 'ö');
    println!(
        "(right)  padded string: {}...",
        vec_pad_right
            .iter()
            .collect::<String>()
            .chars()
            .take(10)
            .collect::<String>()
    );

    let mut vec_pad_center: Vec<char> = original_vec.clone();
    vec_pad_center.shrink_to_fit();
    (&mut vec_pad_center).pad(width, Alignment::Center, 'ö');
    println!(
        "(center) padded string: {}...",
        vec_pad_center
            .iter()
            .collect::<String>()
            .chars()
            .take(10)
            .collect::<String>()
    );
}
