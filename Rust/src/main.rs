// mod _1_two_sum;
// mod _20_valid_parentheses;
// mod _3_longest_substring_without_repeating_characters;
// mod _27_remove_element;
// mod _26_remove_duplicates_from_sorted_array;
// mod _28_find_the_index_of_the_first_occurrence_in_a_string;
// mod _704_binary_search;
// mod _11_container_with_most_water;
// mod _136_single_number;
mod _14_longest_common_prefix;

fn main() {
    let result = _14_longest_common_prefix::Solution::longest_common_prefix(vec![
        "ab".to_string(),
        "a".to_string(),
    ]);
    println!("{:?}", result);
}
