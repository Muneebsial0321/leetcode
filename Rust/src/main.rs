// mod _1_two_sum;
mod _20_valid_parentheses;
fn main() {
    let result = _20_valid_parentheses::Solution::is_valid(String::from("({})[]]"));
    println!("{:?}", result);
}
