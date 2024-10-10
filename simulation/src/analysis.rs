use rayon::prelude::*;
use std::sync::{Arc, Mutex};

use crate::types::{Chain, INF};

fn get_interpolated_k_star(k: &mut Vec<f64>, epsilon: f64) -> f64 {
    k.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let discrete_k_index: usize = ((1.0 - epsilon) * k.len() as f64 - 1.0).floor() as usize;
    let discrete_k = k[discrete_k_index];

    let mut right_index = discrete_k_index + 1;
    let mut left_index = discrete_k_index - 1;

    loop {
        if right_index < k.len() && k[right_index] == discrete_k {
            right_index += 1;
        } else {
            break;
        }
    }
    right_index -= 1;

    loop {
        if left_index > 0 && k[left_index] == discrete_k {
            left_index -= 1;
        } else {
            break;
        }
    }
    let k_star = k[left_index]
        + ((discrete_k_index - left_index) as f64) * (k[right_index] - k[left_index])
            / ((right_index - left_index) as f64);
    k_star
}

fn get_performance(
    honest_work_progresses: &Vec<Chain>,
    adversary_work_progresses: &Vec<Chain>,
) -> (f64, f64, f64) {
    let mut k = INF;
    let mut adversary_index = 0;

    // Go through each honest progress
    'honest_loop: for honest_index in 1..honest_work_progresses.len() {
        let honest_progress = honest_work_progresses[honest_index];
        // Find the adversary progress that immediately follows the honest progress in time
        while adversary_work_progresses[adversary_index].timestamp <= honest_progress.timestamp {
            adversary_index += 1;
            // If there is no adversary progress after the honest progress, we end the simulation here
            if adversary_index == adversary_work_progresses.len() {
                break 'honest_loop;
            }
        }

        // update k if needed
        let previous_adversary_progress = adversary_work_progresses[adversary_index - 1];
        let previous_honest_progress = honest_work_progresses[honest_index - 1];

        if previous_adversary_progress.work >= previous_honest_progress.work {
            // found latest k
            k = honest_progress.work;
        }
        if previous_adversary_progress.work >= honest_progress.work {
            // adversary is ahead, no k found yet
            k = INF;
        }
    }

    let f_work = honest_work_progresses.last().unwrap().work
        / honest_work_progresses.last().unwrap().timestamp;
    let f_height = honest_work_progresses.last().unwrap().height as f64
        / honest_work_progresses.last().unwrap().timestamp;
    (k, f_work, f_height)
}

pub fn get_monte_carlo_performance(
    honest_monte_carlo_progress: &Vec<Vec<Chain>>,
    adversary_monte_carlo_progress: &Vec<Vec<Chain>>,
    epsilon: f64,
) -> (f64, f64, f64) {
    let monte_carlo = honest_monte_carlo_progress.len();
    let f_work_mutex = Arc::new(Mutex::new(0.0));
    let f_height_mutex = Arc::new(Mutex::new(0.0));

    let mut k: Vec<f64> = (0..monte_carlo)
        .into_par_iter()
        .map(|i| {
            let honest_work_progresses = &honest_monte_carlo_progress[i];
            let adversary_work_progresses = &adversary_monte_carlo_progress[i];

            let (k, f_work, f_height) =
                get_performance(honest_work_progresses, adversary_work_progresses);
            let mut local_f_work = f_work_mutex.lock().unwrap();
            let mut local_f_height = f_height_mutex.lock().unwrap();

            *local_f_work += f_work;
            *local_f_height += f_height;
            k
        })
        .collect();

    // k.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // let k_star = k[((monte_carlo as f64) * (1.0 - epsilon)).floor() as usize];
    let k_star = get_interpolated_k_star(&mut k, epsilon);

    let f_work = Arc::try_unwrap(f_work_mutex).unwrap().into_inner().unwrap() / monte_carlo as f64;
    let f_height = Arc::try_unwrap(f_height_mutex)
        .unwrap()
        .into_inner()
        .unwrap()
        / monte_carlo as f64;

    (k_star, f_work, f_height)
}

fn get_k(honest_work_progresses: &Vec<Chain>, adversary_work_progresses: &Vec<Chain>) -> f64 {
    let mut k = INF;
    let mut adversary_index = 0;
    // Go through each honest progress
    for honest_index in 1..honest_work_progresses.len() {
        let honest_progress = honest_work_progresses[honest_index];
        // Find the adversary progress that immediately follows the honest progress in time
        while adversary_work_progresses[adversary_index].timestamp <= honest_progress.timestamp {
            adversary_index += 1;
            // If there is no adversary progress after the honest progress, we end the simulation here
            if adversary_index == adversary_work_progresses.len() {
                return k;
            }
        }

        // update k if needed
        let previous_adversary_progress = adversary_work_progresses[adversary_index - 1];
        let previous_honest_progress = honest_work_progresses[honest_index - 1];

        if previous_adversary_progress.work >= previous_honest_progress.work {
            // found latest k
            k = honest_progress.work;
        }
        if previous_adversary_progress.work >= honest_progress.work {
            // adversary is ahead, no k found yet
            k = INF;
        }
    }
    k
}

pub fn test_get_performance(
    multiple_honest_work_progresses: &Vec<Vec<Chain>>,
    multiple_adversary_work_progresses: &Vec<Vec<Chain>>,
    epsilon: f64,
) -> (f64, f64, f64, f64, Vec<f64>) {
    let monte_carlo = multiple_honest_work_progresses.len();
    let f_work_mutex = Arc::new(Mutex::new(0.0));
    let f_height_mutex = Arc::new(Mutex::new(0.0));

    let mut k: Vec<f64> = (0..monte_carlo)
        .into_par_iter()
        .map(|i| {
            let honest_work_progresses = &multiple_honest_work_progresses[i];
            let adversary_work_progresses = &multiple_adversary_work_progresses[i];

            let local_k = get_k(honest_work_progresses, adversary_work_progresses);
            let mut local_f_work = f_work_mutex.lock().unwrap();
            let mut local_f_height = f_height_mutex.lock().unwrap();

            *local_f_work += honest_work_progresses.last().unwrap().work
                / honest_work_progresses.last().unwrap().timestamp;
            *local_f_height += honest_work_progresses.last().unwrap().height as f64
                / honest_work_progresses.last().unwrap().timestamp;

            local_k
        })
        .collect();

    k.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let k_star = k[((monte_carlo as f64) * (1.0 - epsilon)).floor() as usize];
    let interpolated_k_star = get_interpolated_k_star(&mut k, epsilon);

    let f_work = Arc::try_unwrap(f_work_mutex).unwrap().into_inner().unwrap() / monte_carlo as f64;
    let f_height = Arc::try_unwrap(f_height_mutex)
        .unwrap()
        .into_inner()
        .unwrap()
        / monte_carlo as f64;
    (k_star, interpolated_k_star, f_work, f_height, k)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_k() {
        // Adversary gets in-front at one point but then honest recover

        // Note: arrival_time is set to 0.0 for all chains as it does not matter for this test
        let honest_sample = vec![
            Chain {
                timestamp: 1.0,
                work: 2.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 3.0,
                work: 4.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 4.0,
                work: 6.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 2.0,
                work: 3.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 5.0,
                work: 4.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 6.0,
                work: 5.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];

        assert_eq!(
            get_performance(&honest_sample, &adversary_sample),
            (4.0, 6.0 / 4.0, 3.0 / 4.0)
        );

        // Adversary gets in-front for a while but then honest recovers
        let honest_sample = vec![
            Chain {
                timestamp: 1.0,
                work: 2.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 4.0,
                work: 3.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 5.0,
                work: 6.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 2.0,
                work: 3.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 3.0,
                work: 4.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 6.0,
                work: 5.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];

        assert_eq!(
            get_performance(&honest_sample, &adversary_sample),
            (6.0, 6.0 / 5.0, 3.0 / 5.0)
        );

        // Adversary gets in-front and stays in-front
        let honest_sample = vec![
            Chain {
                timestamp: 2.0,
                work: 1.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 3.0,
                work: 2.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 5.0,
                work: 4.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 2.0,
                work: 2.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 3.0,
                work: 3.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 4.0,
                work: 4.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];

        assert_eq!(
            get_performance(&honest_sample, &adversary_sample),
            (INF, 4.0 / 5.0, 3.0 / 5.0)
        );

        // Honest are in-front at first but then adversary surpasses them
        let honest_sample = vec![
            Chain {
                timestamp: 1.0,
                work: 2.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 4.0,
                work: 3.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 6.0,
                work: 4.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 2.0,
                work: 1.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 3.0,
                work: 3.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 5.0,
                work: 5.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];

        assert_eq!(
            get_performance(&honest_sample, &adversary_sample),
            (INF, 4.0 / 6.0, 3.0 / 6.0)
        );

        // Adversary gets equal work to honest at some point but honest surpasses them after, check off-by-one for equality
        let honest_sample = vec![
            Chain {
                timestamp: 1.0,
                work: 2.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 3.0,
                work: 4.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 5.0,
                work: 6.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];
        let adversary_sample = vec![
            Chain {
                timestamp: 2.0,
                work: 3.0,
                height: 1,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 4.0,
                work: 4.0,
                height: 2,
                arrival_time: 0.0, // Does not matter for this test
            },
            Chain {
                timestamp: 6.0,
                work: 5.0,
                height: 3,
                arrival_time: 0.0, // Does not matter for this test
            },
        ];

        assert_eq!(
            get_performance(&honest_sample, &adversary_sample),
            (6.0, 6.0 / 5.0, 3.0 / 5.0)
        );
    }
}
