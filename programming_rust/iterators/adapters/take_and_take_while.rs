// `take` lets you specify after how many items an iterator
// should stop producing values. `take_while` instead uses
// a closure as a predicate to decide when to stop yielding.

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    for num in v.iter().take(3) {
        println!("{}", num);
    }
    println!("");

    let text = "# Heading 1\n
                \n
                This is a paragraph.";

    for header in text.lines().take_while(|line| !line.is_empty()) {
        println!("{}", header);
    }
}
