use chrono::{DateTime, Duration, Utc};

struct Process {
    id: String,
    arrival_date_time: DateTime<Utc>,
    burst_duration: Duration,
}

pub struct ExecutionLog {
    pid: String,
    start_date_time: DateTime<Utc>,
    end_date_time: DateTime<Utc>,
}

pub fn round_robin(mut processes: Vec<Process>, quantum: Duration) -> Vec<ExecutionLog> {
    let logs: Vec<ExecutionLog> = vec![];
    return logs;
}
