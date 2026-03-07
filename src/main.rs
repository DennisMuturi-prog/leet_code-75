use std::{
    cmp::{Reverse, max, min},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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
                if midpoint == 0 {
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
    pub fn middle_node_normal_approach(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut current = head.as_ref();
        while let Some(current_node) = current {
            count += 1;
            current = current_node.next.as_ref();
        }
        let mid = (count / 2) + 1;
        let mut current = head;
        let mut i = 1;
        while i < mid {
            if let Some(node) = current.take() {
                current = node.next;
            }
            i += 1;
        }
        current
    }
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow_pointer = head.as_ref();
        let mut fast_pointer = head.as_ref();
        loop {
            let fast = match fast_pointer {
                Some(val) => val,
                None => break,
            };
            let fast_next = match fast.next {
                Some(ref val) => val,
                None => break,
            };
            fast_pointer = fast_next.next.as_ref();
            slow_pointer = match slow_pointer {
                Some(val) => val.next.as_ref(),
                None => break,
            }
        }
        match slow_pointer {
            Some(result) => Some(result.clone()),
            None => None,
        }
    }
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());

        for num in nums {
            if seen.contains(&num) {
                return true;
            } else {
                seen.insert(num);
            }
        }
        false
    }
}
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut largest = i32::MIN;
        for num in nums {
            sum += num;
            if num > sum {
                sum = num;
            }
            largest = max(largest, sum);
        }
        largest
    }
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut new_interval = new_interval;
        let mut new_interval_added = false;
        let mut position_found = false;

        for interval in intervals {
            if position_found {
                result.push(interval);
                continue;
            }
            if new_interval[1] < interval[0] {
                result.push(new_interval.clone());
                result.push(interval);

                new_interval_added = true;
                position_found = true;
            } else if new_interval[0] > interval[1] {
                result.push(interval);
            } else {
                new_interval[0] = min(new_interval[0], interval[0]);
                new_interval[1] = max(new_interval[1], interval[1]);
            }
        }
        if !new_interval_added {
            result.push(new_interval);
        }
        result
    }
}

impl Solution {
    pub fn update_matrix_first_approach(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let mut positions = VecDeque::new();
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    positions.push_back((i, j));
                }
            }
        }
        let mut nearest_distances_to_zero: HashMap<(usize, usize), usize> = HashMap::new();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((i, j)) = positions.pop_front() {
            let mut minimum_distance = usize::MAX;
            for (row_change, column_change) in directions {
                let new_row = i as i32 + row_change;
                let new_column = j as i32 + column_change;
                if new_row < 0
                    || new_column < 0
                    || new_row as usize >= mat.len()
                    || new_column as usize >= mat[0].len()
                {
                    continue;
                }
                if mat[new_row as usize][new_column as usize] == 0 {
                    minimum_distance = 0;
                    continue;
                }
                match nearest_distances_to_zero.get(&(new_row as usize, new_column as usize)) {
                    Some(distance) => {
                        minimum_distance = min(minimum_distance, *distance);
                    }
                    None => {
                        positions.push_back((new_row as usize, new_column as usize));
                    }
                }
            }
            let new_distance = if mat[i][j] == 0 {
                0
            } else {
                1 + minimum_distance
            };
            nearest_distances_to_zero.insert((i, j), new_distance);
            mat[i][j] = new_distance as i32;
        }
        mat
    }
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let mut positions = VecDeque::new();
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    positions.push_back((i, j));
                } else {
                    mat[i][j] = i32::MAX;
                }
            }
        }
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((i, j)) = positions.pop_front() {
            for (row_change, column_change) in directions {
                let new_row = i as i32 + row_change;
                let new_column = j as i32 + column_change;
                if new_row < 0
                    || new_column < 0
                    || new_row as usize >= mat.len()
                    || new_column as usize >= mat[0].len()
                {
                    continue;
                }
                if mat[i][j] + 1 < mat[new_row as usize][new_column as usize] {
                    mat[new_row as usize][new_column as usize] = mat[i][j] + 1;

                    positions.push_back((new_row as usize, new_column as usize));
                }
            }
        }
        mat
    }
}
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut min_heap = BinaryHeap::new();
        let mut result = Vec::new();
        for point in points {
            let distance = point[0].pow(2) + point[1].pow(2);
            let point_and_distance = Point {
                distance: Reverse(distance),
                coordinates: point,
            };
            min_heap.push(point_and_distance);
        }
        for _ in 0..k {
            let item = min_heap.pop().unwrap();
            result.push(item.coordinates);
        }
        result
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Point {
    distance: Reverse<i32>,
    coordinates: Vec<i32>,
}

impl Solution {
    pub fn length_of_longest_substring_first_approach(s: String) -> i32 {
        let mut letter_positions = HashMap::new();
        let mut longest_substr_count = 0;
        let mut current_substr_count = 0;
        let mut last_position_with_duplicate = -1;
        for (index, letter) in s.chars().enumerate() {
            match letter_positions.insert(letter, index) {
                Some(first_pos) => {
                    if (first_pos as i32) < last_position_with_duplicate {
                        current_substr_count = index - last_position_with_duplicate as usize;
                    } else {
                        last_position_with_duplicate = first_pos as i32;
                        current_substr_count = index - first_pos;
                    }
                }
                None => {
                    current_substr_count += 1;
                }
            }
            longest_substr_count = max(current_substr_count, longest_substr_count);
        }
        longest_substr_count as i32
    }
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut letters = HashSet::new();
        let mut longest_substr_count = 0;
        let mut left = 0;
        let s = s.chars().collect::<Vec<char>>();
        for right in 0..s.len() {
            while letters.contains(&s[right]) {
                letters.remove(&s[left]);
                left += 1;
            }
            letters.insert(s[right]);
            longest_substr_count = max(right - left + 1, longest_substr_count);
        }
        longest_substr_count as i32
    }
}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut result = Vec::new();
        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let difference = 0 - nums[i];
            let two_sums = Solution::two_sum_for_three_sum(difference, &nums[i + 1..nums.len()]);
            for mut two_sum in two_sums {
                two_sum.push(nums[i]);
                result.push(two_sum);
            }
        }
        result
    }
    pub fn two_sum_for_three_sum(target: i32, nums: &[i32]) -> Vec<Vec<i32>> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        let mut seen_pairs: HashSet<(i32, i32)> = HashSet::new();
        for num in nums {
            let difference = target - num;
            if seen.contains(&difference) {
                if seen_pairs.contains(&(difference, *num)) {
                    continue;
                }
                result.push(vec![difference, *num]);
                seen_pairs.insert((difference, *num));
            } else {
                seen.insert(*num);
            }
        }
        result
    }
    pub fn two_sum_ii_for_three_sum(target: i32, nums: &[i32]) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut left_pointer = 0;
        let mut right_pointer = nums.len() - 1;

        while left_pointer < right_pointer {
            let sum = nums[left_pointer] + nums[right_pointer];
            if sum == target {
                result.push(vec![nums[left_pointer], nums[right_pointer]]);
                left_pointer += 1;
                while left_pointer < right_pointer && nums[left_pointer] == nums[left_pointer - 1] {
                    left_pointer += 1;
                }
                right_pointer -= 1;
                while left_pointer < right_pointer && nums[right_pointer] == nums[right_pointer + 1]
                {
                    right_pointer -= 1;
                }
            } else if sum < target {
                left_pointer += 1;
            } else {
                right_pointer -= 1;
            }
        }
        result
    }
}
impl Solution {
    pub fn two_sum_ii(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left_pointer = 0;
        let mut right_pointer = numbers.len() - 1;

        while left_pointer <= right_pointer {
            let sum = numbers[left_pointer] + numbers[right_pointer];
            if sum == target {
                return vec![(left_pointer + 1) as i32, (right_pointer + 1) as i32];
            } else if sum < target {
                left_pointer += 1;
            } else {
                right_pointer -= 1;
            }
        }
        vec![]
    }
}
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut traversal = VecDeque::new();
        traversal.push_back(root);
        let mut result = Vec::new();

        while !traversal.is_empty() {
            let traversal_len = traversal.len();
            let mut level = Vec::with_capacity(traversal_len);

            for _ in 0..traversal_len {
                let item = traversal.pop_front().unwrap();
                if let Some(val) = item {
                    level.push(val.borrow().val);
                    let left_child = val.borrow_mut().left.take();
                    let right_child = val.borrow_mut().right.take();
                    traversal.push_back(left_child);
                    traversal.push_back(right_child);
                }
            }
            if !level.is_empty() {
                result.push(level);
            }
        }
        result
    }
}
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut operands: Vec<i32> = Vec::new();
        for token in tokens {
            if token == "+" {
                let second_operand = operands.pop().unwrap();
                let first_operand = operands.pop().unwrap();
                let sum = first_operand + second_operand;
                operands.push(sum);
            } else if token == "-" {
                let second_operand = operands.pop().unwrap();
                let first_operand = operands.pop().unwrap();
                let difference = first_operand - second_operand;
                operands.push(difference);
            } else if token == "*" {
                let second_operand = operands.pop().unwrap();
                let first_operand = operands.pop().unwrap();
                let product = first_operand * second_operand;
                operands.push(product);
            } else if token == "/" {
                let second_operand = operands.pop().unwrap();
                let first_operand = operands.pop().unwrap();

                let divison = first_operand / second_operand;
                operands.push(divison);
            } else {
                let num: i32 = token.parse().unwrap();
                operands.push(num);
            }
        }
        operands[0]
    }
}
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adjacency_list = vec![vec![]; num_courses as usize];
        for prerequisite in prerequisites {
            let course = prerequisite[0];
            let prerequisite = prerequisite[1];
            adjacency_list[course as usize].push(prerequisite as usize);
        }
        let mut visited_stack = vec![false; num_courses as usize];
        let mut rec_stack = vec![false; num_courses as usize];
        for i in 0..num_courses as usize {
            if Solution::is_cyclic(i, &adjacency_list, &mut visited_stack, &mut rec_stack) {
                return false;
            }
        }
        true
    }
    fn is_cyclic(
        current_node: usize,
        adjacency_list: &[Vec<usize>],
        visited_stack: &mut [bool],
        rec_stack: &mut [bool],
    ) -> bool {
        if rec_stack[current_node] {
            return true;
        }
        if visited_stack[current_node] {
            return false;
        }
        rec_stack[current_node] = true;
        visited_stack[current_node] = true;

        for neighbour in adjacency_list[current_node].iter() {
            if Solution::is_cyclic(*neighbour, adjacency_list, visited_stack, rec_stack) {
                return true;
            }
        }
        rec_stack[current_node] = false;
        false
    }
}
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut amounts_minimum_coins = vec![(amount + 1); (amount + 1) as usize];
        amounts_minimum_coins[0] = 0;

        for i in 1..=amount as usize {
            let mut minimum = amounts_minimum_coins[i];
            for coin in &coins {
                if *coin as usize <= i {
                    let previous = i - *coin as usize;
                    minimum = min(amounts_minimum_coins[previous], minimum);
                }
            }
            amounts_minimum_coins[i] = 1 + minimum;
        }
        if amounts_minimum_coins[amount as usize] > amount {
            -1
        } else {
            amounts_minimum_coins[amount as usize]
        }
    }
}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix_product = 1;
        let mut result = vec![0; nums.len()];

        for i in 0..nums.len() {
            prefix_product *= nums[i];
            result[i] = prefix_product;
        }
        let mut suffix_product = 1;

        let mut i = (nums.len() - 1) as i32;

        while i >= 0 {
            if i > 0 {
                result[i as usize] = suffix_product * result[(i - 1) as usize];
            } else {
                result[i as usize] = suffix_product;
            }
            suffix_product *= nums[i as usize];
            i -= 1;
        }
        result
    }
}

struct MinStack {
    stack: Vec<NodeInMinStack>,
}
struct NodeInMinStack {
    val: i32,
    min: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push(NodeInMinStack { val, min: val });
        } else {
            let previous_min = self.stack[self.stack.len() - 1].min;
            let new_min = min(previous_min, val);
            self.stack.push(NodeInMinStack { val, min: new_min });
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1].val
    }

    fn get_min(&self) -> i32 {
        self.stack[self.stack.len() - 1].min
    }
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let min = None;
        let max = None;
        Solution::is_valid_node_bst(root, min, max)
    }
    fn is_valid_node_bst(
        node: Option<Rc<RefCell<TreeNode>>>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> bool {
        match node {
            Some(current_node) => {
                let val = current_node.borrow().val;
                if let Some(minimum) = min
                    && val <= minimum
                {
                    return false;
                }
                if let Some(maximum) = max
                    && val >= maximum
                {
                    return false;
                }

                let left_child = current_node.borrow_mut().left.take();
                let right_child = current_node.borrow_mut().right.take();

                if !Solution::is_valid_node_bst(left_child, min, Some(val)) {
                    return false;
                }
                Solution::is_valid_node_bst(right_child, Some(val), max)
            }
            None => true,
        }
    }
}
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut number_of_islands = 0;
        let mut grid = grid;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    number_of_islands += 1;
                    Solution::visit_island_parts(&mut grid, i, j);
                }
            }
        }

        number_of_islands
    }
    pub fn visit_island_parts(grid: &mut Vec<Vec<char>>, row: usize, column: usize) {
        grid[row][column] = '2';

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for (row_change, column_change) in directions {
            let new_row = row as i32 - row_change;
            let new_column = column as i32 - column_change;
            if new_row < 0
                || new_column < 0
                || new_row as usize >= grid.len()
                || new_column as usize >= grid[0].len()
            {
                continue;
            }
            if grid[new_row as usize][new_column as usize] != '1' {
                continue;
            }

            Solution::visit_island_parts(grid, new_row as usize, new_column as usize);
        }
    }
}
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut traversal_list = VecDeque::new();
        let mut empty_cells = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 2 {
                    traversal_list.push_back((i, j));
                } else if grid[i][j] == 0 {
                    empty_cells += 1;
                }
            }
        }
        let mut no_of_seconds = -1;
        let mut grid = grid;
        let mut no_of_rotten_fruits = 0;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while !traversal_list.is_empty() {
            let length_of_level = traversal_list.len();
            no_of_rotten_fruits += length_of_level;

            for _ in 0..length_of_level {
                let (i, j) = traversal_list.pop_front().unwrap();
                for (row_change, column_change) in directions {
                    let new_row = i as i32 - row_change;
                    let new_column = j as i32 - column_change;
                    if new_row < 0
                        || new_column < 0
                        || new_row as usize >= grid.len()
                        || new_column as usize >= grid[0].len()
                    {
                        continue;
                    }
                    if grid[new_row as usize][new_column as usize] != 1 {
                        continue;
                    }
                    grid[new_row as usize][new_column as usize] = 2;

                    traversal_list.push_back((new_row as usize, new_column as usize));
                }
            }
            no_of_seconds += 1;
        }
        if no_of_rotten_fruits != (grid.len() * grid[0].len()) - empty_cells {
            -1
        } else {
            if no_of_seconds == -1 {
                0
            } else {
                no_of_seconds
            }
        }
    }
}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        let minimum_index = left;
        if nums[nums.len() - 1] > nums[0] {
            left = 0;
            right = nums.len() - 1;
        } else if target < nums[0] {
            left = minimum_index;
            right = nums.len() - 1;
        } else {
            left = 0;
            right = minimum_index.saturating_sub(1);
        }
        while left <= right {
            let midpoint = left + (right - left) / 2;
            if nums[midpoint] == target {
                return midpoint as i32;
            } else if target < nums[midpoint] {
                if midpoint == 0 {
                    return -1;
                }
                right = midpoint - 1;
            } else {
                left = midpoint + 1;
            }
        }
        -1
    }
}
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut path = Vec::new();
        let mut paths = Vec::new();
        Solution::dfs_combination_sum(target,0, &candidates, &mut path, &mut paths);

        paths
    }
    pub fn dfs_combination_sum(
        target: i32,
        index:usize,
        candidates: &[i32],
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 || index==candidates.len() {
            return;
        }
        if target == 0 {
            paths.push(path.clone());
            return;
        } else {
            path.push(candidates[index]);
            Solution::dfs_combination_sum(target-candidates[index], index, candidates, path, paths);
            path.pop();
            Solution::dfs_combination_sum(target, index+1, candidates, path, paths);
        }
    }
}
