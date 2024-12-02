use core::panic;
use std::{collections::VecDeque, ops::Not};

use rand::Rng;

#[derive(serde::Deserialize, serde::Serialize, Clone, Default, Debug, PartialEq)]
pub enum CPUStatus {
    Working,
    #[default]
    Idle,
}

#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug, PartialEq)]
#[serde(default)]
pub struct Job {
    pub job_name: String,
    pub arrival_time: u32,
    pub needed_cpu_cycle: u32,
    pub remaining_cpu_cycle: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
}

pub fn job_builder(job_count: u32) -> Vec<Job> {
    let mut jobs = Vec::new();
    for i in 0..job_count {
        jobs.push(Job {
            job_name: format!("{}", (b'A' + i as u8) as char),
            arrival_time: 0,
            needed_cpu_cycle: 0,
            remaining_cpu_cycle: 0,
            completion_time: 0,
            turnaround_time: 0,
        });
    }
    jobs
}

pub fn randomize_jobs(jobs: Vec<Job>) -> Vec<Job> {
    let mut rng = rand::thread_rng();
    let mut scheduled_jobs = jobs.clone();

    for job in &mut scheduled_jobs {
        job.arrival_time = rng.gen_range(0..10);
        job.needed_cpu_cycle = rng.gen_range(1..10);
    }
    let output = scheduled_jobs.clone();
    output
}

/// Returns Jobs and Timeline
pub fn process_scheduler(
    algorithm: String,
    mut jobs: Vec<Job>,
) -> (Vec<Job>, Vec<(String, u32, u32)>) {
    let mut rng = rand::thread_rng();
    let mut timeline: Vec<(String, u32, u32)> = Vec::new();
    let mut job_name;
    let mut start_time;
    let mut end_time;
    // process_scheduling_algorithm
    // 1: FCFS
    // 2: SJN
    // 3: SRN
    // 4: Round Robin

    // INITIALIZE JOBS
    let mut x: u32 = 0;
    for job in &mut jobs {
        x += job.needed_cpu_cycle;
        job.remaining_cpu_cycle = job.needed_cpu_cycle; // Initialize remaining_cpu_cycle
    }
    let expected_cpu_max: u32 = x;
    let mut current_job: Job = jobs[0].clone(); // Initialize to first job
    let mut to_return_jobs: Vec<Job> = jobs.clone();

    //
    let algorithm_num = match algorithm.as_str() {
        "Random" => 0,
        "First Come First Serve (FCFS)" => 1,
        "Shortest Job Next (SJN)" => 2,
        "Shortest Remaining Time (SRN)" => 3,
        "Round Robin" => 4,
        _ => -1, // Unknown, program will panic
    };

    // Sort Jobs by Arrival Time
    jobs.sort_by(|a, b| a.arrival_time.partial_cmp(&b.arrival_time).unwrap());

    // Randomize (Ignore Parameters)
    if algorithm_num == 0 {
        for job in &mut jobs {
            let current_timeline_index = timeline.len();
            // println!("{}", current_timeline_index);
            job_name = job.job_name.to_string(); // idk why this works
            if current_timeline_index == 0 {
                start_time = 0;
            } else {
                start_time = timeline[current_timeline_index - 1].2;
            }

            end_time = &start_time + rng.gen_range(0..5);
            timeline.push(({ job_name }, { start_time }, { end_time }));
            job.arrival_time = rng.gen_range(0..10);
            job.needed_cpu_cycle = rng.gen_range(1..10);
        }
    }
    // First Come First Serve
    else if algorithm_num == 1 {
        for job in &mut jobs {
            timeline.push(({ job.job_name.to_string() }, { job.arrival_time }, {
                job.needed_cpu_cycle
            }))
            // TODO: Modify jobs, and update numbers
        }
    }
    // Shortest Job Next (SJN)
    else if algorithm_num == 2 {
        let mut queue: VecDeque<Job> = Vec::new().into(); // Contains jobs that have arrived but are in queue
        let mut finished_jobs: Vec<Job> = vec![];
        let mut cpu_counter: u32 = 0;

        let total_job_count = jobs.len();
        let mut finished_jobs_count: usize = 0;
        let mut arrived_jobs_count: usize = 0;

        let mut cpu_status: CPUStatus = CPUStatus::Idle;

        to_return_jobs = vec![]; // Reset
                                 // Example
                                 //jn:  a    b   c
                                 //ar:  1    2   3
                                 //cc:  5    4   2
                                 // Needs reordering of queue every job arrival

        // 1) Job Arrives, Put Job in Queue
        // 2) If Something is in Progress, Put Job in Queue
        // 3) If something cpu is waiting, put next job in queue

        // Get job (already ordered by arrival time)
        println!("Total Job Count: {}", total_job_count);
        while finished_jobs_count < total_job_count {
            // Handle Job Arrival, if there are still jobs pending
            if arrived_jobs_count < total_job_count {
                //                                       vv prevents crashing when arrival times are misconfigured
                if jobs[arrived_jobs_count].arrival_time <= cpu_counter {
                    println!(
                        "JOB ARRIVED: {} // CPU_COUNTER: {}",
                        jobs[arrived_jobs_count].job_name, cpu_counter
                    );
                    queue.push_back(jobs[arrived_jobs_count].clone());
                    arrived_jobs_count += 1;
                }
            }

            // If CPU idle and queue is not empty && PROCESS JOB
            if cpu_status == CPUStatus::Idle && queue.is_empty().not() {
                queue
                    .make_contiguous()
                    .sort_by(|a, b| a.needed_cpu_cycle.partial_cmp(&b.needed_cpu_cycle).unwrap());
                let pop_back = queue.pop_front();
                if pop_back != None {
                    current_job = pop_back.expect("Unexpected: pop_back is None.");
                }
                println!("JOB WORKING: {}", current_job.job_name);
                cpu_status = CPUStatus::Working;
            }
            // IF CPU is Working
            // Not else if to allow cpu to work upon arrival
            if cpu_status == CPUStatus::Working {
                if current_job.remaining_cpu_cycle > 0 {
                    current_job.remaining_cpu_cycle -= 1;
                }
                // If Job just finished; Cannot be else if
                if current_job.remaining_cpu_cycle == 0 {
                    println!(
                        "JOB FINISHED: {} // CPU_COUNTER: {}",
                        current_job.job_name, cpu_counter
                    );
                    // Will not work for other algorithms
                    timeline.push((
                        { current_job.job_name.to_string() },
                        { (cpu_counter + 1) - current_job.needed_cpu_cycle },
                        { cpu_counter + 1 },
                    )); // Return already processed
                    finished_jobs.push(current_job.clone());
                    cpu_status = CPUStatus::Idle;
                    finished_jobs_count += 1;

                    println!(
                        "FINISHED JOBS COUNT: {} // CPU_COUNTER: {}",
                        finished_jobs_count, cpu_counter
                    );
                }
            }
            cpu_counter += 1;

            if cpu_counter > expected_cpu_max * 2 {
                panic!("cpu_counter is greater than DOUBLE of EXPECTED_CPU_MAX")
            }
        }
    }
    // Shortest Remaining Time (SRT)
    // else if algorithm_num == 3 {

    // }
    // Round Robin
    // else if algorithm_num == 4 {
    // }
    else {
        panic!("Unexpected: algorithm_num is -1")
    }

    // Return Jobs and Timeline
    (to_return_jobs, timeline)
}

// Function Tests
// TODO: Implement tests for checking

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sjn_works() {
        let mut jobs: Vec<Job> = vec![];
        jobs.push(Job {
            job_name: "A".to_string(),
            arrival_time: 0,
            needed_cpu_cycle: 5,
            remaining_cpu_cycle: 5,
            completion_time: 0,
            turnaround_time: 0,
        });
        jobs.push(Job {
            job_name: "B".to_string(),
            arrival_time: 2,
            needed_cpu_cycle: 5,
            remaining_cpu_cycle: 5,
            completion_time: 0,
            turnaround_time: 0,
        });
        jobs.push(Job {
            job_name: "C".to_string(),
            arrival_time: 3,
            needed_cpu_cycle: 3,
            remaining_cpu_cycle: 3,
            completion_time: 0,
            turnaround_time: 0,
        });
        let (_, timeline) = process_scheduler("Shortest Job Next (SJN)".to_string(), jobs);
        assert_eq!(
            timeline,
            [
                ("A".to_string(), 0, 5),
                ("C".to_string(), 5, 8),
                ("B".to_string(), 8, 13)
            ]
        )
    }
}
