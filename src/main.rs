use solutions::solution_001::Solution001;

mod solutions;

fn main() {
    let input: Vec<i32> = vec![3, 2, 4];
    let target: i32 = 6;

    let result = Solution001::two_sum(input, target);

    for ele in result {
        print!("{}", &ele);
    }
}
