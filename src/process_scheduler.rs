use rand::Rng;

#[derive(serde::Deserialize, serde::Serialize, Clone, Default, Debug)]
pub enum JobStatus {
    Complete,
    InProgress,
    #[default]
    NotStarted,
}
#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[serde(default)]
pub struct Job {
    pub job_name: String,
    pub arrival_time: u32,
    pub cpu_cycle: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
    pub job_status: JobStatus,
}

pub fn job_builder(job_count: u32) -> Vec<Job> {
    let mut jobs = Vec::new();
    for i in 0..job_count {
        jobs.push(Job {
            job_name: format!("{}", (b'A' + i as u8) as char),
            arrival_time: 0,
            cpu_cycle: 0,
            completion_time: 0,
            turnaround_time: 0,
            job_status: JobStatus::NotStarted,
        });
    }
    jobs
}

pub fn randomize_jobs(jobs: Vec<Job>) -> Vec<Job> {
    let mut rng = rand::thread_rng();
    let mut scheduled_jobs = jobs.clone();

    for job in &mut scheduled_jobs {
        job.arrival_time = rng.gen_range(0..10);
        job.cpu_cycle = rng.gen_range(1..10);
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

    let algorithm_num = match algorithm.as_str() {
        "Random" => 0,
        "First Come First Serve (FCFS)" => 1,
        "Shortest Job Next (SJN)" => 2,
        "Shortest Remaining Time (SRN)" => 3,
        "Round Robin" => 4,
        _ => -1, // Unknown
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
            job.cpu_cycle = rng.gen_range(1..10);
        }
    }
    // First Come First Serve
    else if algorithm_num == 1 {
        for job in &mut jobs {
            job_name = job.job_name.to_string(); // idk why this works
            timeline.push(({ job_name }, { job.arrival_time }, { job.cpu_cycle }))
        }
    }
    // Shortest Job Next (SJN)
    else if algorithm_num == 2 {
        let mut queue: Vec<Job> = vec![];
        let mut counter: u32 = 0;
        let job_count: u32 = jobs.len().try_into().unwrap();
        let finished_jobs_count: u32 = 0;

        // Example
        //jn:  a    b   c
        //ar:  1    2   3
        //cc:  5    4   2
        // Needs reordering of queue every job arrival

        // Get job (already ordered by arrival time)

        for 0..job_count {
            // We can't reorder completed/in-progress jobs.
            job.job_status = JobStatus::InProgress;
        }
    }
    // Shortest Remaining Time (SRT)
    else if algorithm_num == 3 {
    }
    // Round Robin
    else if algorithm_num == 4 {
    }

    // Return Jobs and Timeline
    (jobs, timeline)
}

// Function Tests
// TODO: Implement tests for checking

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
