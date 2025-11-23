// mod _1_two_sum;
// mod _20_valid_parentheses;
// mod _3_longest_substring_without_repeating_characters;
// mod _27_remove_element;
mod _26_remove_duplicates_from_sorted_array;
// mod _28_find_the_index_of_the_first_occurrence_in_a_string;
fn main() {
    let result = _26_remove_duplicates_from_sorted_array::Solution::remove_duplicates(&mut vec![1,1,2]);
    println!("{:?}", result);
}
