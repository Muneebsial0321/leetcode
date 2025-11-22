// mod _1_two_sum;
// mod _20_valid_parentheses;
// mod _3_longest_substring_without_repeating_characters;
mod _27_remove_element;
// mod _28_find_the_index_of_the_first_occurrence_in_a_string;
fn main() {
    let result = _27_remove_element::Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2],2);
    println!("{:?}", result);
}
