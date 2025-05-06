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

pub trait Scheduler {
    fn schedule(&self, processes: Vec<Process>) -> Vec<ExecutionLog>;
}

pub struct RoundRobin {
    pub quantum: Duration,
}

pub struct ShortestJobFirst;

impl Scheduler for RoundRobin {
    fn schedule(&self, processes: Vec<Process>) -> Vec<ExecutionLog> {
        let mut execution_logs = vec![];
        let mut process_queue = processes;
        process_queue.sort_by_key(|p| p.arrival_date_time);

        let mut current_date_time = process_queue[0].arrival_date_time;
        let mut remaining_process_queue = process_queue;

        while !remaining_process_queue.is_empty() {
            let mut next = vec![];

            for process in remaining_process_queue {
                let duration = process.burst_duration.min(self.quantum);
                let start = current_date_time;
                let end = current_date_time + duration;

                execution_logs.push(ExecutionLog {
                    pid: process.id.clone(),
                    start_date_time: start,
                    end_date_time: end,
                });

                current_date_time = end;

                if process.burst_duration > self.quantum {
                    next.push(Process {
                        burst_duration: process.burst_duration - self.quantum,
                        ..process
                    });
                }
            }

            remaining_process_queue = next;
        }

        return execution_logs;
    }
}

impl Scheduler for ShortestJobFirst {
    fn schedule(&self, processes: Vec<Process>) -> Vec<ExecutionLog> {
        let mut execution_logs = vec![];
        let mut process_queue = processes;
        process_queue.sort_by_key(|p| p.arrival_date_time);

        let mut current_date_time = process_queue[0].arrival_date_time;
        let mut remaining_process_queue = process_queue;

        while !remaining_process_queue.is_empty() {
            let mut ready_processes: Vec<Process> = remaining_process_queue
                .iter()
                .filter(|p| p.arrival_date_time <= current_date_time)
                .cloned()
                .collect();

            if ready_processes.is_empty() {
                current_date_time = remaining_process_queue[0].arrival_date_time;
                continue;
            }

            ready_processes.sort_by_key(|p| p.burst_duration);
            let shortest_process = ready_processes[0].clone();
            let start = current_date_time;
            let end = start + shortest_process.burst_duration;

            execution_logs.push(ExecutionLog {
                pid: shortest_process.id.clone(),
                start_date_time: start,
                end_date_time: end,
            });

            current_date_time = end;
            remaining_process_queue.retain(|p| p.id != shortest_process.id);
        }

        return execution_logs;
    }
}
