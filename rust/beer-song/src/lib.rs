pub fn verse(n: i32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n.to_string(), n.to_string(), (n - 1).to_string()),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n.to_string(), n.to_string(), (n - 1).to_string())
    }
}

pub fn sing(from: i32, to: i32) -> String {
    let mut verses: String = String::new();

    for n in (to..(from + 1)).rev() {
        verses.push_str(&verse(n));
        if n != to {
            verses.push_str(&"\n");
        }
    }

    verses
}