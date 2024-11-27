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

pub fn process_scheduler(process_scheduling_algorithm: u32, jobs: Vec<Job>) -> Vec<Job> {
    // process_scheduling_algorithm
    // 1: FCFS
    // 2: SJN
    // 3: SRN
    // 4: Round Robin
    let mut rng = rand::thread_rng();
    let mut scheduled_jobs = jobs;

    for job in &mut scheduled_jobs {
        job.arrival_time = rng.gen_range(0..10);
        job.cpu_cycle = rng.gen_range(1..10);
    }

    scheduled_jobs
}
