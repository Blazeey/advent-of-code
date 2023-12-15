fn hash(text: &str) -> u32 {
    text.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

fn main() {
    let file_content = include_str!("../../input2.txt");
    // println!("{:?}", 'a' as u32);
    let ans: u32 = file_content.split(',').map(hash).sum();
    println!("Ans: {:?}", ans);
}
