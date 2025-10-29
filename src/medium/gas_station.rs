pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut sum = 0;

        let mut index_reduced = 0;
        let mut reduced_indices: Vec<usize> = Vec::new();

        let mut start_candidates = Vec::new();
        let mut start_buffers = Vec::new();
        let mut diff;

        for i in 0..gas.len() {
            if gas[i] == cost[i] {
                continue;
            }
            diff = gas[i] - cost[i];

            if diff >= 0 {
                start_candidates.push(index_reduced);
                start_buffers.push(sum);
            }
            reduced_indices.push(i);
            index_reduced += 1;

            sum += diff;
        }

        if sum < 0 {
            return -1;
        }

        println!("start indices {:?}", start_candidates);
        println!("start buffers {:?}", start_buffers);

        for i in 0..start_candidates.len() {
            if Self::check_cicruit(
                &gas,
                &cost,
                start_candidates[i],
                start_buffers[i],
                &reduced_indices,
            ) {
                return reduced_indices[start_candidates[i]] as i32;
            }
        }
        -1
    }

    fn check_cicruit(
        gas: &Vec<i32>,
        cost: &Vec<i32>,
        start: usize,
        buffer: i32,
        reduced_indices: &Vec<usize>,
    ) -> bool {
        let mut sum = 0;
        println!("check start {}", start);
        for i in start..reduced_indices.len() {
            sum += gas[reduced_indices[i]] - cost[reduced_indices[i]];
            println!("  i {} sum {}", reduced_indices[i], sum);
            if sum < 0 {
                return false;
            }
        }
        sum + buffer >= 0
    }
}
