use std::collections::HashMap;

pub struct Solution001 {}

impl Solution001 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (index, elem) in nums.into_iter().enumerate() {
            if map.contains_key(&elem) {
                return vec![map[&elem] as i32, index as i32];
            } else {
                map.insert(target - elem, index);
            }
        }
        return vec![];
    }
}

#[cfg(test)]
mod test {
    use super::Solution001;


    #[test]
    fn test_work() {
        let input: Vec<i32> = vec![3, 2, 4];
        let target: i32 = 6;

        let result = Solution001::two_sum(input, target);

        for ele in result {
            print!("{}", &ele);
        }
    }
}
