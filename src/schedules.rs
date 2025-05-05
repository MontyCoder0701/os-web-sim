use chrono::{DateTime, Duration, Utc};

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
    let logs: Vec<ExecutionLog> = vec![];
    return logs;
}
