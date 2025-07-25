use padder::{Alignment, MutableSource, pad_mut};

fn main() {
    let width: usize = 12;
    let original_s = String::from("alonso");

    let mut s_pad_left = original_s.clone();
    s_pad_left.shrink_to_fit();
    (&mut s_pad_left).pad(width, Alignment::Left, '🌊');
    assert_eq!("alonso🌊🌊🌊🌊🌊🌊", s_pad_left,);
    println!("(left)   padded: '{s_pad_left}'");

    let mut s_pad_right = original_s.clone();
    s_pad_right.shrink_to_fit();
    pad_mut(&mut s_pad_right, width, Alignment::Right, '🦆');
    assert_eq!("🦆🦆🦆🦆🦆🦆alonso", s_pad_right,);
    println!("(right)  padded: '{s_pad_right}'");

    let mut s_pad_center = original_s.clone();
    s_pad_center.shrink_to_fit();
    (&mut s_pad_center).pad(width, Alignment::Center, '🌴');
    assert_eq!("🌴🌴🌴alonso🌴🌴🌴", s_pad_center,);
    println!("(center) padded: '{s_pad_center}'");
}
