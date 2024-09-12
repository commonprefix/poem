use std::collections::VecDeque;

use rand::rngs::ThreadRng;
use rand_distr::{Exp, Distribution};

#[derive(Debug, Clone)]
struct Progress {
    timestamp: f64,
    work: f64,
}

type Sample = Vec<Progress>;
type Samples = Vec<Sample>;

const INF: f64 = f64::INFINITY;

// def sample_honest(g, max_weight, get_work=lambda: 1):
//   weight_improvements = [(0,0)] # (arrival_time, weight)
//   block_time = 0
//   heaviest_chain_weight = 0
//   receive_events = deque()
//   while weight_improvements[-1][1] < max_weight:
//     block_time += np.random.exponential(1/g)

//     # Before processing the newly mined block first process all received blocks before it
//     while len(receive_events) > 0:
//       arrival_time, weight = receive_events[0]
//       if arrival_time > block_time:
//         break
//       heaviest_chain_weight = max(heaviest_chain_weight, weight)
//       receive_events.popleft()

//     block_arrival_time = block_time + 1 # Δ = 1
//     this_block_weight = get_work()
//     new_chain_weight = heaviest_chain_weight + this_block_weight
//     receive_events.append((block_arrival_time, new_chain_weight)) # the optimal adversary delays as much as allowed

//     if weight_improvements[-1][1] < new_chain_weight:
//       weight_improvements.append((block_time, new_chain_weight))
//   return weight_improvements


fn sample_honest(g: f64, max_weight: f64, mut get_work: impl FnMut() -> f64) -> Sample {
    let mut weight_improvements = vec![Progress { timestamp: 0.0, work: 0.0 }];
    let mut block_time = 0.0;
    let mut heaviest_chain_weight: f64 = 0.0;
    let mut receive_events: VecDeque<(f64, f64)> = VecDeque::new();

    let exponential_distribution = Exp::new(g).unwrap();
    let mut rng = rand::thread_rng();

    while weight_improvements.last().unwrap().work < max_weight {
        block_time += exponential_distribution.sample(&mut rng);

        // Before processing the newly mined block first process all received blocks before it
        while let Some((arrival_time, weight)) = receive_events.front() {
            if *arrival_time > block_time {
                break;
            }
            heaviest_chain_weight = heaviest_chain_weight.max(*weight);
            receive_events.pop_front();
        }

        let block_arrival_time = block_time + 1.0; // Δ = 1
        let this_block_weight = get_work();
        let new_chain_weight = heaviest_chain_weight + this_block_weight;
        receive_events.push_back((block_arrival_time, new_chain_weight)); // the optimal adversary delays as much as allowed

        if weight_improvements.last().unwrap().work < new_chain_weight {
            weight_improvements.push(Progress { timestamp: block_time, work: new_chain_weight });
        }
    }

    weight_improvements
}

fn sample_adversary(g: f64, max_weight: f64, mut get_work: impl FnMut() -> f64) -> Sample {
    let mut block_time = 0.0;
    let mut block_weight = 0.0;

    let mut weight_improvements = vec![Progress { timestamp: 0.0, work: 0.0 }];

    let exponential_distribution = Exp::new(g).unwrap();
    let mut rng = rand::thread_rng();

    while weight_improvements.last().unwrap().work < max_weight {
        block_time += exponential_distribution.sample(&mut rng);
        block_weight += get_work();
        weight_improvements.push(Progress { timestamp: block_time, work: block_weight });
    }

    return weight_improvements;
}

fn sample_multiple_adversaries(g: f64, max_weight: f64, mut get_work: impl FnMut() -> f64, monte_carlo: usize) -> Samples {
    let mut samples = Vec::new();
    for _i in 0..monte_carlo {
        samples.push(sample_adversary(g, max_weight, &mut get_work));
    }
    samples
}


fn entropy_function(gamma: f64) -> impl FnMut() -> f64 {
    let exponential_distribution = Exp::new(std::f64::consts::LN_2).unwrap();
    let mut rng = rand::thread_rng();

    move || {
        exponential_distribution.sample(&mut rng) + gamma
    }
}

fn sample_multiple_honest(g: f64, max_weight: f64, mut get_work: impl FnMut() -> f64, monte_carlo: usize) -> Samples {
    let mut samples = Vec::new();
    for _i in 0..monte_carlo {
        samples.push(sample_honest(g, max_weight, &mut get_work));
    }
    samples
}


fn get_k(honest_sample: Sample, adversary_sample: Sample) -> Option<f64> {
    if honest_sample.last().unwrap().timestamp >= adversary_sample.last().unwrap().timestamp {
        return Some(INF);
    }

    let mut j = adversary_sample.len() - 1;
    for i in (1..honest_sample.len()).rev() {
        while honest_sample[i].timestamp < adversary_sample[j - 1].timestamp {
            j -= 1;
        }
        if honest_sample[i - 1].work <= adversary_sample[j - 1].work {
            return Some(honest_sample[i].work)
        }
    }

    return None;
}

fn ternary_search(left: f64, right: f64, function: fn(f64) -> f64) -> f64 {
    todo!()
}

fn get_latency(honest_samples: Samples, adversary_samples: Samples, epsilon: f64) -> f64 {
    let mut potential_k = vec![];
    let mut f = 0.0;
    let monte_carlo = honest_samples.len();
    for i in 0..monte_carlo {
        potential_k.push(get_k(honest_samples[i].clone(), adversary_samples[i].clone()).unwrap());
        f += honest_samples[i].last().unwrap().work / honest_samples[i].last().unwrap().timestamp;
    }
    potential_k.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let k = potential_k[(monte_carlo as f64 * epsilon).ceil() as usize];
    f = f / monte_carlo as f64;
    let latency = k / f;

    latency
}

fn get_latency_for_beta_g_gamma(beta: f64, g: f64, gamma: f64, monte_carlo: usize) -> f64 {
    let mut get_entropy = entropy_function(gamma);
    // println!("gamma: {}", entropy_function(gamma)());
    let honest_samples = sample_multiple_honest(g, 400.0, &mut get_entropy, monte_carlo);
    let adversary_samples = sample_multiple_adversaries(g * beta / (1.0 - beta), 400.0, get_entropy, monte_carlo);

    get_latency(honest_samples, adversary_samples, 0.1)
}


fn main() {
    // let sample_honest = sample_honest(0.3, 10.0, || 1.0);
    // println!("{:?}", sample_honest);
    let start = std::time::Instant::now();

    for gamma in 0..120 {
        println!("{gamma}: {:?}", get_latency_for_beta_g_gamma(0.3, 0.3, gamma as f64, 100000));
    }

    // sample_multiple_honest(0.3, 400.0, get_entropy(20 as f64), 100000);
    for _i in 0..1000000 {
        // sample_honest(0.3, 10.0, || 1.0);
    }
    let duration = start.elapsed().as_secs_f64();
    println!("Time elapsed: {:.2} seconds", duration);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_k() {
        let honest_sample = vec![
            Progress { timestamp: 0.0, work: 0.0 },
            Progress { timestamp: 2.0, work: 1.0 },
            Progress { timestamp: 3.0, work: 2.0 }
        ];
        let adversary_sample = vec![
            Progress { timestamp: 0.0, work: 0.0 },
            Progress { timestamp: 1.0, work: 1.0 },
            Progress { timestamp: 4.0, work: 2.0 }
        ];
        assert_eq!(get_k(honest_sample, adversary_sample).unwrap(), 2.0);

        println!("-----");
        let honest_sample = vec![
            Progress { timestamp: 0.0, work: 0.0 },
            Progress { timestamp: 1.0, work: 2.0 },
            Progress { timestamp: 4.0, work: 3.0 }
        ];
        let adversary_sample = vec![
            Progress { timestamp: 0.0, work: 0.0 },
            Progress { timestamp: 2.0, work: 1.0 },
            Progress { timestamp: 3.0, work: 3.0 }
        ];
        assert_eq!(get_k(honest_sample, adversary_sample).unwrap(), INF);

        println!("-----");
        let honest_sample = vec![
            Progress { timestamp: 0.0, work: 0.0 },
            Progress { timestamp: 1.0, work: 1.0 },
            Progress { timestamp: 3.0, work: 2.0 }
        ];
        let adversary_sample = vec![
            Progress { timestamp: 0.0, work: 0.0 },
            Progress { timestamp: 2.0, work: 1.0 },
            Progress { timestamp: 4.0, work: 2.0 }
        ];

        assert_eq!(get_k(honest_sample, adversary_sample).unwrap(), 2.0);

        println!("-----");
        let honest_sample = vec![
            Progress { timestamp: 0.0, work: 0.0 },
            Progress { timestamp: 1.0, work: 1.0 },
            Progress { timestamp: 3.0, work: 3.0 }
        ];
        let adversary_sample = vec![
            Progress { timestamp: 0.0, work: 0.0 },
            Progress { timestamp: 2.0, work: 0.5 },
            Progress { timestamp: 4.0, work: 2.0 }
        ];

        assert_eq!(get_k(honest_sample, adversary_sample).unwrap(), 1.0);
    }
}