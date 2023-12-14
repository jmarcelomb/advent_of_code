#[derive(Clone, Debug)]
struct UseCase {
    hold_time: usize,
    distance: usize,
}

impl UseCase {
    fn default() -> Self {
        UseCase {
            hold_time: 0,
            distance: 0,
        }
    }
}

fn generate_usecases(race_time: usize) -> Vec<UseCase> {
    let mut use_cases: Vec<UseCase> = vec![UseCase::default(); race_time];

    for (time, use_case) in use_cases.iter_mut().enumerate() {
        use_case.hold_time = time;
        use_case.distance = (race_time - use_case.hold_time) * use_case.hold_time;
    }
    return use_cases;
}

fn get_win_usecases(use_cases: &Vec<UseCase>, target_distance: usize) -> Vec<UseCase> {
    use_cases
        .iter()
        .filter(|usecase| usecase.distance > target_distance)
        .map(|use_case| use_case.clone())
        .collect()
}

fn part1(times: &[usize], distances: &[usize]) -> usize {
    let mut sum: usize = 1;

    for (i, distance) in distances.iter().enumerate() {
        let use_cases = generate_usecases(times[i]);
        let win_use_cases = get_win_usecases(&use_cases, *distance);
        if win_use_cases.len() != 0 {
            sum *= win_use_cases.len();
        }
    }
    sum
}

fn main() {
    let easy_times = vec![7, 15, 30];
    let easy_distances = vec![9, 40, 200];
    let part1_small_res = part1(&easy_times, &easy_distances);
    assert_eq!(part1_small_res, 288);

    let times = vec![45, 97, 72, 95];
    let distances = vec![305, 1062, 1110, 1695];

    let part1_res = part1(&times, &distances);

    let times = vec![45977295];
    let distances = vec![305106211101695];

    let part2_res = part1(&times, &distances);

    println!("Part 1 sum: {part1_res}");
    println!("Part 2 sum: {part2_res}");
}
