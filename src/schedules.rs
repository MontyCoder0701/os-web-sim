use chrono::{DateTime, Duration, Utc};

#[derive(Clone)]
pub struct Process {
    pub id: String,
    pub arrival_date_time: DateTime<Utc>,
    pub burst_duration: Duration,
}

pub struct ExecutionLog {
    pub pid: String,
    pub start_date_time: DateTime<Utc>,
    pub end_date_time: DateTime<Utc>,
}

pub fn round_robin(processes: Vec<Process>, quantum: Duration) -> Vec<ExecutionLog> {
    let mut execution_logs = vec![];
    let mut process_queue = processes;
    process_queue.sort_by_key(|p| p.arrival_date_time);

    let mut current_date_time = process_queue[0].arrival_date_time;
    let mut remaining_process_queue = process_queue;

    while !remaining_process_queue.is_empty() {
        let mut next = vec![];

        for process in remaining_process_queue {
            let duration = process.burst_duration.min(quantum);
            let start = current_date_time;
            let end = current_date_time + duration;

            execution_logs.push(ExecutionLog {
                pid: process.id.clone(),
                start_date_time: start,
                end_date_time: end,
            });

            current_date_time = end;

            if process.burst_duration > quantum {
                next.push(Process {
                    burst_duration: process.burst_duration - quantum,
                    ..process
                });
            }
        }

        remaining_process_queue = next;
    }

    return execution_logs;
}
