pub struct SolutionHanota;

impl SolutionHanota {
    pub fn tower_of_hanoi(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        fn move_pillar(plates: usize, a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
            if plates == 1 {
                let plate = a.pop();
                match plate {
                    Some(plate) => {
                        c.push(plate);
                        return;
                    }
                    None => return,
                }
            }
            move_pillar(plates - 1, a, c, b);
            move_pillar(1, a, b, c);
            move_pillar(plates - 1, b, a, c);
        }

        move_pillar(a.len(), a, b, c);
    }
}

#[cfg(test)]
mod test {
    use super::SolutionHanota;

    #[test]
    fn test_work() {
        let mut a = vec![2, 1, 0];
        let mut b: Vec<i32> = vec![];
        let mut c: Vec<i32> = vec![];

        SolutionHanota::tower_of_hanoi(&mut a, &mut b, &mut c);
        println!("{:?}", c);
    }
}
