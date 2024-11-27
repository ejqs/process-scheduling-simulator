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

pub fn process_scheduler(algorithm: String, mut jobs: Vec<Job>) -> (Vec<Job>, Vec<(String, u32)>) {
    let mut rng = rand::thread_rng();
    let mut timeline: Vec<(String, u32)> = Vec::new();

    // process_scheduling_algorithm
    // 1: FCFS
    // 2: SJN
    // 3: SRN
    // 4: Round Robin

    let algorithm_num = match algorithm.as_str() {
        "First Come First Serve (FCFS)" => 1,
        "Shortest Job Next (SJN)" => 2,
        "Shortest Remaining Time (SRN)" => 3,
        "Round Robin" => 4,
        _ => panic!("Unknown scheduling algorithm"),
    };

    if algorithm_num == 1 {
        timeline = vec![
            ("a".to_string(), 0),
            ("b".to_string(), 2),
            ("c".to_string(), 6),
            ("d".to_string(), 11),
        ]
    }

    (jobs, timeline)
}
