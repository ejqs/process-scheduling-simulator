use rand::Rng;

#[derive(serde::Deserialize, serde::Serialize, Default, Clone)]
#[serde(default)]
pub struct Job {
    pub job_name: String,
    pub arrival_time: u32,
    pub cpu_cycle: u32,
    pub completion_time: u32,
    pub turnaround_time: u32,
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

    // Randomize
    if algorithm_num == 0 {
        let mut job_name;
        let mut start_time;
        let mut end_time;
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

        // timeline = vec![
        //     ("a".to_string(), 0),
        //     ("b".to_string(), 2),
        //     ("c".to_string(), 6),
        //     ("d".to_string(), 11),
        // ]
    }

    (jobs, timeline)
}
