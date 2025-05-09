use chrono::{DateTime, Duration, Utc};

#[derive(Clone)]
pub struct Process {
    pub id: String,
    pub arrival_date_time: DateTime<Utc>,
    pub burst_duration: Duration,
    pub priority: u32,
}

pub struct ExecutionLog {
    pub pid: String,
    pub start_date_time: DateTime<Utc>,
    pub end_date_time: DateTime<Utc>,
}

pub trait Scheduling {
    fn schedule(&self, processes: Vec<Process>) -> Vec<ExecutionLog>;
}

pub struct RoundRobin {
    pub quantum: Duration,
}

pub struct FirstComeFirstServed;

pub struct ShortestJobFirst;

pub struct Priority;

impl Scheduling for RoundRobin {
    fn schedule(&self, processes: Vec<Process>) -> Vec<ExecutionLog> {
        let mut execution_logs = vec![];
        let mut cloned_processes = processes;
        cloned_processes.sort_by_key(|p| p.arrival_date_time);

        let mut current_date_time = cloned_processes[0].arrival_date_time;

        while !cloned_processes.is_empty() {
            cloned_processes.retain_mut(|process| {
                let duration = process.burst_duration.min(self.quantum);
                let start = current_date_time;
                let end = current_date_time + duration;

                execution_logs.push(ExecutionLog {
                    pid: process.id.clone(),
                    start_date_time: start,
                    end_date_time: end,
                });

                current_date_time = end;

                if process.burst_duration <= self.quantum {
                    return false;
                }

                process.burst_duration -= self.quantum;
                return true;
            });
        }

        return execution_logs;
    }
}

impl Scheduling for FirstComeFirstServed {
    fn schedule(&self, processes: Vec<Process>) -> Vec<ExecutionLog> {
        let mut execution_logs = vec![];
        let mut cloned_processes = processes;
        cloned_processes.sort_by_key(|p| p.arrival_date_time);

        let mut current_date_time = cloned_processes[0].arrival_date_time;

        for process in cloned_processes {
            if process.arrival_date_time > current_date_time {
                current_date_time = process.arrival_date_time;
            }

            let start = current_date_time;
            let end = start + process.burst_duration;

            execution_logs.push(ExecutionLog {
                pid: process.id,
                start_date_time: start,
                end_date_time: end,
            });

            current_date_time = end;
        }

        return execution_logs;
    }
}

impl Scheduling for ShortestJobFirst {
    fn schedule(&self, processes: Vec<Process>) -> Vec<ExecutionLog> {
        let mut execution_logs = vec![];
        let mut cloned_processes = processes;
        cloned_processes.sort_by_key(|p| p.burst_duration);

        let mut current_date_time = cloned_processes[0].arrival_date_time;

        for process in cloned_processes {
            if process.arrival_date_time > current_date_time {
                current_date_time = process.arrival_date_time;
            }

            let start = current_date_time;
            let end = start + process.burst_duration;

            execution_logs.push(ExecutionLog {
                pid: process.id,
                start_date_time: start,
                end_date_time: end,
            });

            current_date_time = end;
        }

        return execution_logs;
    }
}

impl Scheduling for Priority {
    fn schedule(&self, processes: Vec<Process>) -> Vec<ExecutionLog> {
        let mut execution_logs = vec![];
        let mut current_date_time = processes[0].arrival_date_time;

        let mut cloned_processes = processes;
        cloned_processes.sort_by_key(|p| p.priority);

        for process in cloned_processes {
            if process.arrival_date_time > current_date_time {
                current_date_time = process.arrival_date_time;
            }

            let start = current_date_time;
            let end = start + process.burst_duration;

            execution_logs.push(ExecutionLog {
                pid: process.id,
                start_date_time: start,
                end_date_time: end,
            });

            current_date_time = end;
        }

        return execution_logs;
    }
}
