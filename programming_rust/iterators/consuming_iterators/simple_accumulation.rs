// Besides consuming iterators using for loops
// or just calling the `next` method you can use
// consuming methods provided by the `Iterator`
// trait.

fn main() {
    let nums = [1, 2, 3, 4, 5];
    println!("{:?}", nums);

    let count = nums.iter().count();
    println!("count: {}", count);

    let sum: i32 = nums.iter().sum();
    println!("sum: {}", sum);

    let product: i32 = nums.iter().product();
    println!("product: {}", product);
}
