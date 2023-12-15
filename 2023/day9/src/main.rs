use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_next_array(current_array: &Vec<i32>) -> Vec<i32> {
    let mut new_arr: Vec<i32> = vec![0; current_array.len() - 1];

    for i in 0..current_array.len() - 1 {
        new_arr[i] = current_array[i + 1] - current_array[i];
    }
    new_arr
}

fn part1(file_name: &str) -> i32 {
    let lines = read_lines(file_name);

    let mut sum: i32 = 0;

    for line in lines.clone().into_iter() {
        let values: Vec<i32> = line
            .split(" ")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        let mut arrays_tree: Vec<Vec<i32>> = vec![values];

        let mut next_array = get_next_array(&arrays_tree.last().unwrap());
        arrays_tree.push(next_array);

        while arrays_tree.last().unwrap().iter().all(|x| *x == 0) == false {
            next_array = get_next_array(&arrays_tree.last().unwrap());
            arrays_tree.push(next_array);
        }

        for i in (1..arrays_tree.len()).rev() {
            let last_value = arrays_tree[i].last().unwrap();
            let previous_last_value = arrays_tree[i - 1].last().unwrap();
            let value = *previous_last_value + *last_value;
            arrays_tree[i - 1].push(value);
        }
        sum += arrays_tree[0].last().unwrap();
    }
    sum
}

fn part2(file_name: &str) -> i32 {
    let lines = read_lines(file_name);

    let mut sum: i32 = 0;

    for line in lines.clone().into_iter() {
        let values: Vec<i32> = line
            .split(" ")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        let mut arrays_tree: Vec<Vec<i32>> = vec![values];

        let mut next_array = get_next_array(&arrays_tree.last().unwrap());
        arrays_tree.push(next_array);

        while arrays_tree.last().unwrap().iter().all(|x| *x == 0) == false {
            next_array = get_next_array(&arrays_tree.last().unwrap());
            arrays_tree.push(next_array);
        }

        for i in (1..arrays_tree.len()).rev() {
            let first_value = arrays_tree[i][0];
            let previous_first_value = arrays_tree[i - 1][0];
            let value = previous_first_value - first_value;
            arrays_tree[i - 1].insert(0, value);
        }
        sum += arrays_tree[0][0];
    }
    sum
}

fn main() {
    let part1_small_res = part1("small_input.txt");
    assert_eq!(part1_small_res, 114);

    let part1_res = part1("input.txt");

    let part2_res = part2("input.txt");

    println!("Part 1 sum: {part1_res}");
    println!("Part 2 sum: {part2_res}");
}
