fn main() {
    let christmas_gifts = [
        "Gift 1", "Gift 2", "Gift 3", "Gift 4", "Gift 5", "Gift 6", "Gift 7", "Gift 8", "Gift 9",
        "Gift 10", "Gift 11", "Gift 12",
    ];

    for day in 1..christmas_gifts.len() + 1 {
        print!("Day {day}: ");
        for gift in (0..day).rev() {
            print!(
                "{}{}",
                christmas_gifts[gift],
                if gift != 0 { ", " } else { "" }
            );
        }
        println!();
    }
}
