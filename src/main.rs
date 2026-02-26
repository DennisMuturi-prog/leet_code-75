use std::{
    cmp::{Reverse, max},
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

fn main() {
    println!("Hello, world!");
}
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();
        let mut result = Vec::new();
        for (index, num) in nums.into_iter().enumerate() {
            let difference = target - num;
            if seen.contains_key(&difference) {
                let first_index = seen.get(&difference).unwrap();
                result.push(*first_index as i32);
                result.push(index as i32);
                return result;
            } else {
                seen.insert(num, index);
            }
        }
        result
    }
}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut letter_stack = Vec::new();
        let mut letters = s.chars();
        let combinations = HashMap::from([('}', '{'), (')', '('), (']', '[')]);
        match letters.next() {
            Some(first_letter) => {
                if first_letter == ')' || first_letter == '}' || first_letter == ']' {
                    return false;
                }
                letter_stack.push(first_letter);
            }
            None => {
                return false;
            }
        }
        for letter in letters {
            if letter == '(' || letter == '{' || letter == '[' {
                letter_stack.push(letter);
            } else {
                match letter_stack.pop() {
                    Some(top_letter) => match combinations.get(&letter) {
                        Some(complement) => {
                            if &top_letter != complement {
                                return false;
                            }
                        }
                        None => {
                            return false;
                        }
                    },
                    None => {
                        return false;
                    }
                };
            }
        }
        letter_stack.is_empty()
    }
}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut head = None;
        let mut first = list1;
        let mut second = list2;

        if let Some(mut first_node) = first.take()
            && let Some(mut second_node) = second.take()
        {
            if first_node.val < second_node.val {
                first = first_node.next.take();
                head = Some(first_node);
                second = Some(second_node);
            } else {
                second = second_node.next.take();
                head = Some(second_node);
                first = Some(first_node);
            }
        }
        let mut current_node = head.as_mut();
        loop {
            match first.take() {
                Some(mut first_node) => match second.take() {
                    Some(mut second_node) => {
                        if first_node.val < second_node.val {
                            match current_node.take() {
                                Some(node) => {
                                    first = first_node.next.take();
                                    node.next = Some(first_node);
                                    current_node = node.next.as_mut();
                                }
                                None => {
                                    break;
                                }
                            };
                            second = Some(second_node);
                        } else {
                            match current_node.take() {
                                Some(node) => {
                                    second = second_node.next.take();
                                    node.next = Some(second_node);
                                    current_node = node.next.as_mut();
                                }
                                None => {
                                    break;
                                }
                            };
                            first = Some(first_node);
                        }
                    }
                    None => {
                        match current_node.take() {
                            Some(node) => {
                                node.next = Some(first_node);
                            }
                            None => {
                                break;
                            }
                        };
                        break;
                    }
                },
                None => {
                    match current_node.take() {
                        Some(node) => {
                            node.next = second;
                        }
                        None => {
                            break;
                        }
                    };
                    break;
                }
            }
        }
        head
    }
}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut higher = 0;
        let mut lowest_finder: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for price in prices {
            if let Some(lowest) = lowest_finder.peek() {
                let difference = price - lowest.0;
                if difference > higher {
                    higher = difference;
                }
            }
            lowest_finder.push(Reverse(price));
        }
        higher
    }
}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }
        let filtered: Vec<char> = s
            .chars()
            .filter(|item| item.is_alphanumeric())
            .map(|a| a.to_ascii_lowercase())
            .collect();
        if filtered.len() <= 1 {
            return true;
        }

        let mut starting = 0;
        let mut ending = filtered.len() - 1;

        while ending > starting {
            if filtered[starting] != filtered[ending] {
                return false;
            } else {
                starting += 1;
                ending -= 1;
            }
        }

        true
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let inverted_left = Solution::invert_tree(node.borrow_mut().left.take());
                let inverted_right = Solution::invert_tree(node.borrow_mut().right.take());
                node.borrow_mut().left = inverted_right;
                node.borrow_mut().right = inverted_left;
                Some(node)
            }
            None => None,
        }
    }
}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut all_letters = [0; 26];

        for letter in s.chars() {
            all_letters[letter as usize - 97] += 1;
        }
        for letter in t.chars() {
            all_letters[letter as usize - 97] -= 1;
        }

        for count in all_letters {
            if count != 0 {
                return false;
            }
        }
        true
    }
}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut starting = 0;
        let mut ending = nums.len() - 1;

        while starting <= ending {
            let midpoint = starting + (ending - starting) / 2;
            if nums[midpoint] == target {
                return midpoint as i32;
            } else if target < nums[midpoint] {
                if ending == 0 {
                    return -1;
                }
                ending = midpoint - 1;
            } else {
                starting = midpoint + 1;
            }
        }
        -1
    }
}
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let starting_point_colour = image[sr as usize][sc as usize];
        if starting_point_colour == color {
            return image;
        }
        Solution::flood_fill_dsf(
            &mut image,
            sr as usize,
            sc as usize,
            starting_point_colour,
            color,
        );
        image
    }
    pub fn flood_fill_dsf(
        image: &mut Vec<Vec<i32>>,
        row: usize,
        column: usize,
        starting_point_colour: i32,
        color: i32,
    ) {
        image[row][column] = color;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (row_change, column_change) in directions {
            let new_row = row as i32 - row_change;
            let new_column = column as i32 - column_change;
            if new_row < 0
                || new_column < 0
                || new_row as usize >= image.len()
                || new_column as usize >= image[0].len()
            {
                continue;
            }
            if image[new_row as usize][new_column as usize] != starting_point_colour {
                continue;
            }
            if image[new_row as usize][new_column as usize] == color {
                continue;
            }
            Solution::flood_fill_dsf(
                image,
                new_row as usize,
                new_column as usize,
                starting_point_colour,
                color,
            );
        }
    }
}
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => match p {
                Some(ref p_node) => match q {
                    Some(ref q_node) => {
                        let value_of_node = node.borrow().val;
                        let p_val = p_node.borrow().val;
                        let q_val = q_node.borrow().val;

                        if p_val < value_of_node && q_val < value_of_node {
                            return Solution::lowest_common_ancestor(
                                node.borrow_mut().left.take(),
                                p,
                                q,
                            );
                        } else if p_val > value_of_node && q_val > value_of_node {
                            return Solution::lowest_common_ancestor(
                                node.borrow_mut().right.take(),
                                p,
                                q,
                            );
                        } else {
                            Some(node)
                        }
                    }
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
}
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (_, balance) = Solution::height_and_balance(root);
        balance
    }

    pub fn height_and_balance(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        match node {
            Some(tree_node) => {
                let (left_height, left_balance) =
                    Solution::height_and_balance(tree_node.borrow_mut().left.take());
                if !left_balance {
                    return (0, false);
                }

                let (right_height, right_balance) =
                    Solution::height_and_balance(tree_node.borrow_mut().right.take());
                if !right_balance {
                    return (0, false);
                }
                let difference = left_height.abs_diff(right_height);
                if difference > 1 {
                    return (0, false);
                }
                (1 + max(left_height, right_height), true)
            }
            None => (0, true),
        }
    }
}

struct MyQueue {
    front: Vec<i32>,
    back: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            front: Vec::new(),
            back: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.front.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.back.is_empty() {
            while let Some(val) = self.front.pop() {
                self.back.push(val);
            }
        }

        self.back.pop().unwrap()
    }

    fn peek(&self) -> i32 {
        if self.back.is_empty() {
            self.front[0]
        } else {
            self.back[self.back.len() - 1]
        }
    }

    fn empty(&self) -> bool {
        self.back.is_empty() && self.front.is_empty()
    }
}
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut starting = 1;
        let mut ending = n;

        while starting < ending {
            let midpoint = starting + (ending - starting) / 2;
            if self.is_bad_version(midpoint) {
                ending = midpoint;
            } else {
                starting = midpoint + 1;
            }
        }
        starting
    }
    pub fn is_bad_version(&self, version: i32) -> bool {
        todo!()
    }
}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if magazine.len() < ransom_note.len() {
            return false;
        }
        let mut all_letters = [0; 26];

        for letter in magazine.chars() {
            all_letters[letter as usize - 97] += 1;
        }
        for letter in ransom_note.chars() {
            if all_letters[letter as usize - 97] == 0 {
                return false;
            }
            all_letters[letter as usize - 97] -= 1;
        }

        true
    }
}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 || n == 2 {
            return n;
        }
        let mut previous = 1;
        let mut next = 2;
        for i in 3..=n {
            let temp = next;
            next += previous;
            previous = temp;
        }
        next
    }
}
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut letters = HashMap::new();

        let s = s.chars();

        for letter in s {
            letters
                .entry(letter)
                .and_modify(|a| {
                    *a += 1;
                })
                .or_insert(1);
        }
        let mut anchor_chosen = false;
        let mut longest = 0;
        for count in letters.values() {
            if count % 2 == 0 {
                longest += count;
            } else {
                if !anchor_chosen {
                    longest += 1;
                    anchor_chosen = true;
                }
                longest += count - 1;
            }
        }
        longest
    }
}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = None;
        let mut current = head;
        while let Some(mut current_node) = current {
            current = current_node.next.take();
            current_node.next = previous;
            previous = Some(current_node);
        }
        previous
    }
}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let nums_len = nums.len();
        let mut numbers_count = HashMap::new();
        for num in nums {
            numbers_count
                .entry(num)
                .and_modify(|a| {
                    *a += 1;
                })
                .or_insert(1);
        }
        let mut highest_count = 1;
        for (key, val) in numbers_count {
            if val > nums_len / 2 {
                highest_count = key;
            }
        }
        highest_count
    }
}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_len = a.len();
        let b_len = b.len();
        let mut a_digits = a.chars().rev();
        let mut b_digits = b.chars().rev();
        let mut carry = 0;
        let mut result = String::new();

        for _ in 0..max(a_len, b_len) {
            let digit_a: usize = match a_digits.next() {
                Some(digit) => digit.to_string().parse().unwrap(),
                None => 0,
            };
            let digit_b: usize = match b_digits.next() {
                Some(digit) => digit.to_string().parse().unwrap(),
                None => 0,
            };
            let sum = digit_a + digit_b + carry;
            if sum == 3 {
                carry = 1;
                result.push('1');
            } else if sum == 2 {
                carry = 1;
                result.push('0');
            } else {
                carry = 0;
                result.push_str(&sum.to_string());
            }
        }
        if carry == 1 {
            result.push('1');
        }
        result.chars().rev().collect()
    }
}
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_height = 0;
        Solution::height(root, &mut max_height);
        max_height as i32
    }
    pub fn height(node: Option<Rc<RefCell<TreeNode>>>, max_height: &mut usize) -> usize {
        match node {
            Some(tree_node) => {
                let left_height = Solution::height(tree_node.borrow_mut().left.take(), max_height);

                let right_height =
                    Solution::height(tree_node.borrow_mut().right.take(), max_height);
                *max_height = max(*max_height, left_height + right_height);

                1 + max(left_height, right_height)
            }
            None => 0,
        }
    }
}
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {}
}
