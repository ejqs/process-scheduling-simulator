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
    pub remaining_cpu_cycle: u32, // rem cpu needs to be initialized with needed; code has no proper setter/getter
    pub completion_time: u32,
    pub turnaround_time: u32,
}

fn return_job_name(i: usize) -> String {
    let mut n = i + 1;
    let mut name = String::new();
    while n > 0 {
        let rem = (n - 1) % 26;
        name.insert(0, (b'A' + rem as u8) as char);
        n = (n - 1) / 26;
    }
    name
}
pub fn job_builder(old_jobs: &Vec<Job>, job_count: u32) -> Vec<Job> {
    let mut jobs = Vec::new();
    // https://stackoverflow.com/a/45344045

    let mut job_built: usize = 0;
    for mut old_job in old_jobs.clone() {
        old_job.job_name = return_job_name(job_built);
        jobs.push(old_job);
        job_built += 1;
    }

    let remaining_job_to_build = job_count as i32 - jobs.len() as i32;
    // To build
    // FIXME: Properly test and increment letters
    if remaining_job_to_build > 0 {
        for _ in 0..remaining_job_to_build {
            jobs.push(Job {
                // job_name: format!("{}", (b'A' + (jobs.len() as u8 + i as u8)) as char),
                job_name: return_job_name(job_built as usize),
                arrival_time: 0,
                needed_cpu_cycle: 1,
                remaining_cpu_cycle: 1,
                completion_time: 0,
                turnaround_time: 0,
            });
            job_built += 1;
        }
    } else {
        // Destroy
        for _ in 0..remaining_job_to_build.abs() {
            jobs.pop();
        }
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
    time_quantum: u32,
) -> (Vec<Job>, Vec<(String, u32, u32)>) {
    let mut rng = rand::thread_rng();
    let mut timeline: Vec<(String, u32, u32)> = Vec::new();
    let mut job_name;
    let mut start_time;
    let mut end_time;

    let mut queue: VecDeque<Job> = Vec::new().into(); // Contains jobs that have arrived but are in queue
    let mut finished_jobs: Vec<Job> = vec![];
    let mut cpu_counter: u32 = 0;

    let total_job_count = jobs.len();
    let mut finished_jobs_count: usize = 0;
    let mut arrived_jobs_count: usize = 0;

    let mut cpu_status: CPUStatus = CPUStatus::Idle;

    // process_scheduling_algorithm
    // 1: FCFS
    // 2: SJN
    // 3: SRN
    // 4: Round Robin

    // INITIALIZE JOBS
    let mut cpu_time_max: u32 = 0;
    let mut arrival_max: u32 = 0;
    for job in &mut jobs {
        cpu_time_max += job.needed_cpu_cycle;
        job.remaining_cpu_cycle = job.needed_cpu_cycle; // Initialize remaining_cpu_cycle
        if job.arrival_time > arrival_max {
            arrival_max = job.arrival_time;
        }
    }

    // DONE: Fix crash for when arrival time outlives expected_cpu_max
    let expected_cpu_max: u32 = if cpu_time_max > arrival_max {
        cpu_time_max
    } else {
        arrival_max
    };
    let mut current_job: Job = jobs[0].clone(); // Initialize to first job
    let mut to_return_jobs: Vec<Job> = jobs.clone();

    //
    let algorithm_num = match algorithm.as_str() {
        "Random" => 0,
        "First Come First Serve (FCFS)" => 1,
        "Shortest Job Next (SJN)" => 2,
        "Shortest Remaining Time (SRT)" => 3,
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
    else if algorithm_num == 3 {
        let mut time_start_work_uninterrupted: u32 = 0;
        let mut time_end_work_uninterrupted: u32;
        println!("Total Job Count: {}", total_job_count);
        while finished_jobs_count < total_job_count {
            // Handle Job Arrival, if there are still jobs pending
            if arrived_jobs_count < total_job_count {
                //                                       vv prevents bugs when arrival times are misconfigured
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
                queue.make_contiguous().sort_by(|a, b| {
                    a.remaining_cpu_cycle
                        .partial_cmp(&b.remaining_cpu_cycle)
                        .unwrap()
                });
                // TODO: Not sure if necessary
                let pop_back = queue.pop_front();
                if pop_back != None {
                    current_job = pop_back.expect("Unexpected: pop_back is None.");
                }
                time_start_work_uninterrupted = cpu_counter;
                cpu_status = CPUStatus::Working;
            }

            // IF CPU is Working and There's a Job with Shorter CPU Time
            if cpu_status == CPUStatus::Working {
                // SORT by accending remaining_cpu_cycle
                queue.make_contiguous().sort_by(|a, b| {
                    a.remaining_cpu_cycle
                        .partial_cmp(&b.remaining_cpu_cycle)
                        .unwrap()
                });
                // INTERRUPT JOB if queue job has more remaining
                if queue.is_empty().not() && current_job.remaining_cpu_cycle > 0 {
                    if current_job.remaining_cpu_cycle > queue[0].remaining_cpu_cycle {
                        time_end_work_uninterrupted = cpu_counter;
                        timeline.push((
                            { current_job.job_name.to_string() },
                            { time_start_work_uninterrupted },
                            { time_end_work_uninterrupted },
                        ));

                        queue.push_back(current_job);
                        current_job = queue.pop_front().unwrap();
                        time_start_work_uninterrupted = cpu_counter;
                    }
                }
                // WORKING
                if current_job.remaining_cpu_cycle > 0 {
                    println!("JOB WORKING: {}", current_job.job_name);
                    current_job.remaining_cpu_cycle -= 1;
                }

                // IF JOB JUST FINISHED
                if current_job.remaining_cpu_cycle == 0 {
                    println!(
                        "JOB FINISHED: {} // CPU_COUNTER: {}",
                        current_job.job_name, cpu_counter
                    );

                    // Will not work for other algorithms
                    // Return already processed
                    time_end_work_uninterrupted = cpu_counter + 1;
                    timeline.push((
                        { current_job.job_name.to_string() },
                        { time_start_work_uninterrupted },
                        { time_end_work_uninterrupted },
                    ));

                    finished_jobs.push(current_job.clone());
                    // if !queue.is_empty() {
                    //     queue.make_contiguous().sort_by(|a, b| {
                    //         a.remaining_cpu_cycle
                    //             .partial_cmp(&b.remaining_cpu_cycle)
                    //             .unwrap()
                    //     });
                    //     time_start_work_uninterrupted = cpu_counter + 1;
                    //     current_job = queue.pop_front().unwrap();
                    // }

                    finished_jobs_count += 1;

                    println!(
                        "FINISHED JOBS COUNT: {} // CPU_COUNTER: {}",
                        finished_jobs_count, cpu_counter
                    );
                }
            }

            if current_job.remaining_cpu_cycle == 0 {
                cpu_status = CPUStatus::Idle;
            }

            cpu_counter += 1;

            if cpu_counter > expected_cpu_max * 2 {
                panic!("cpu_counter is greater than DOUBLE of EXPECTED_CPU_MAX")
            }
        }
    }
    // Round Robin
    else if algorithm_num == 4 {
        let mut time_start_work_uninterrupted: u32 = 0;
        let mut time_end_work_uninterrupted: u32;
        let mut just_popped: bool;
        println!("Total Job Count: {}", total_job_count);
        while finished_jobs_count < total_job_count {
            // Handle Job Arrival, if there are still jobs pending
            if arrived_jobs_count < total_job_count {
                //                                       vv prevents bugs when arrival times are misconfigured
                if jobs[arrived_jobs_count].arrival_time <= cpu_counter {
                    println!(
                        "JOB ARRIVED: {} // CPU_COUNTER: {}",
                        jobs[arrived_jobs_count].job_name, cpu_counter
                    );
                    queue.push_back(jobs[arrived_jobs_count].clone());
                    arrived_jobs_count += 1;
                }
            }
            just_popped = false;
            // If CPU idle and queue is not empty && PROCESS JOB
            if cpu_status == CPUStatus::Idle && queue.is_empty().not() {
                // TODO: Not sure if necessary
                let pop_back = queue.pop_front();
                if pop_back != None {
                    current_job = pop_back.expect("Unexpected: pop_back is None.");
                }
                time_start_work_uninterrupted = cpu_counter;
                cpu_status = CPUStatus::Working;
                just_popped = true;
            }

            if cpu_status == CPUStatus::Working {
                // Handle preempting jobs on time quantum
                // e.g. time_quantum = 3;; 0, 1, 2, 3, 4, 5, 6
                //                                  ^        ^
                if cpu_counter % time_quantum == 0 && !just_popped {
                    if queue.is_empty().not() {
                        time_end_work_uninterrupted = cpu_counter; // preempt happens before work
                        timeline.push((
                            { current_job.job_name.to_string() },
                            { time_start_work_uninterrupted },
                            { time_end_work_uninterrupted },
                        ));
                        queue.push_back(current_job);
                        current_job = queue.pop_front().unwrap();
                        time_start_work_uninterrupted = cpu_counter; // new start
                    }
                }

                // WORKING
                if current_job.remaining_cpu_cycle > 0 {
                    println!("JOB WORKING: {}", current_job.job_name);
                    current_job.remaining_cpu_cycle -= 1;
                }

                // IF JOB JUST FINISHED
                if current_job.remaining_cpu_cycle == 0 {
                    println!(
                        "JOB FINISHED: {} // CPU_COUNTER: {}",
                        current_job.job_name, cpu_counter
                    );

                    // Will not work for other algorithms
                    // Return already processed
                    time_end_work_uninterrupted = cpu_counter + 1;
                    timeline.push((
                        { current_job.job_name.to_string() },
                        { time_start_work_uninterrupted },
                        { time_end_work_uninterrupted },
                    ));

                    finished_jobs.push(current_job.clone());
                    // if !queue.is_empty() {
                    //     queue.make_contiguous().sort_by(|a, b| {
                    //         a.remaining_cpu_cycle
                    //             .partial_cmp(&b.remaining_cpu_cycle)
                    //             .unwrap()
                    //     });
                    //     time_start_work_uninterrupted = cpu_counter + 1;
                    //     current_job = queue.pop_front().unwrap();
                    // }

                    finished_jobs_count += 1;

                    println!(
                        "FINISHED JOBS COUNT: {} // CPU_COUNTER: {}",
                        finished_jobs_count, cpu_counter
                    );
                }
            }
            if current_job.remaining_cpu_cycle == 0 {
                cpu_status = CPUStatus::Idle;
            }
            cpu_counter += 1;

            if cpu_counter > expected_cpu_max * 2 {
                panic!("cpu_counter is greater than DOUBLE of EXPECTED_CPU_MAX")
            }
        }
    } else {
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
        let (_, timeline) = process_scheduler("Shortest Job Next (SJN)".to_string(), jobs, 0);
        assert_eq!(
            timeline,
            [
                ("A".to_string(), 0, 5),
                ("C".to_string(), 5, 8),
                ("B".to_string(), 8, 13)
            ]
        )
    }

    #[test]
    fn srt_works() {
        let mut jobs: Vec<Job> = vec![];
        jobs.push(Job {
            job_name: "A".to_string(),
            arrival_time: 0,
            needed_cpu_cycle: 6,
            remaining_cpu_cycle: 6,
            completion_time: 0,
            turnaround_time: 0,
        });
        jobs.push(Job {
            job_name: "B".to_string(),
            arrival_time: 1,
            needed_cpu_cycle: 3,
            remaining_cpu_cycle: 3,
            completion_time: 0,
            turnaround_time: 0,
        });
        jobs.push(Job {
            job_name: "C".to_string(),
            arrival_time: 2,
            needed_cpu_cycle: 1,
            remaining_cpu_cycle: 1,
            completion_time: 0,
            turnaround_time: 0,
        });
        jobs.push(Job {
            job_name: "D".to_string(),
            arrival_time: 3,
            needed_cpu_cycle: 4,
            remaining_cpu_cycle: 4,
            completion_time: 0,
            turnaround_time: 0,
        });
        let (_, timeline) = process_scheduler("Shortest Remaining Time (SRT)".to_string(), jobs, 0);
        assert_eq!(
            timeline,
            [
                ("A".to_string(), 0, 1),
                ("B".to_string(), 1, 2),
                ("C".to_string(), 2, 3),
                ("B".to_string(), 3, 5),
                ("D".to_string(), 5, 9),
                ("A".to_string(), 9, 14),
            ]
        )
    }

    #[test]
    fn rr_works() {
        let mut jobs: Vec<Job> = vec![];
        jobs.push(Job {
            job_name: "A".to_string(),
            arrival_time: 0,
            needed_cpu_cycle: 8,
            remaining_cpu_cycle: 8,
            completion_time: 0,
            turnaround_time: 0,
        });
        jobs.push(Job {
            job_name: "B".to_string(),
            arrival_time: 1,
            needed_cpu_cycle: 4,
            remaining_cpu_cycle: 4,
            completion_time: 0,
            turnaround_time: 0,
        });
        jobs.push(Job {
            job_name: "C".to_string(),
            arrival_time: 2,
            needed_cpu_cycle: 9,
            remaining_cpu_cycle: 9,
            completion_time: 0,
            turnaround_time: 0,
        });
        jobs.push(Job {
            job_name: "D".to_string(),
            arrival_time: 3,
            needed_cpu_cycle: 5,
            remaining_cpu_cycle: 5,
            completion_time: 0,
            turnaround_time: 0,
        });
        let (_, timeline) = process_scheduler("Round Robin".to_string(), jobs, 4);
        assert_eq!(
            timeline,
            [
                ("A".to_string(), 0, 4),
                ("B".to_string(), 4, 8),
                ("C".to_string(), 8, 12),
                ("D".to_string(), 12, 16),
                ("A".to_string(), 16, 20),
                ("C".to_string(), 20, 24),
                ("D".to_string(), 24, 25),
                ("C".to_string(), 25, 26),
            ]
        )
    }
}
