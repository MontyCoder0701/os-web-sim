use chrono::{Datelike, Duration, TimeZone, Utc};
use scheduling::{
    FirstComeFirstServed, Priority, Process, RoundRobin, Scheduling, ShortestJobFirst,
};
use yew::prelude::*;

mod scheduling;

#[function_component]
fn App() -> Html {
    let processes = vec![
        Process {
            id: "P1".to_string(),
            arrival_date_time: Utc::now(),
            burst_duration: Duration::seconds(5),
            priority: 1,
        },
        Process {
            id: "P2".to_string(),
            arrival_date_time: Utc::now(),
            burst_duration: Duration::seconds(3),
            priority: 2,
        },
        Process {
            id: "P3".to_string(),
            arrival_date_time: Utc::now(),
            burst_duration: Duration::seconds(6),
            priority: 3,
        },
    ];

    let rr = RoundRobin {
        quantum: Duration::seconds(2),
    };
    let fcfs = FirstComeFirstServed;
    let sjf = ShortestJobFirst;
    let pr = Priority;

    let rr_logs = rr.schedule(&processes);
    let fcfs_logs = fcfs.schedule(&processes);
    let sjf_logs = sjf.schedule(&processes);
    let pr_logs = pr.schedule(&processes);

    html! {
        <div style="padding: 32px;">
            { render_gantt_chart("Round Robin", &rr_logs) }
            { render_gantt_chart("First Come First Served", &fcfs_logs) }
            { render_gantt_chart("Shortest Job First", &sjf_logs) }
            { render_gantt_chart("Priority", &pr_logs) }

            <footer style="margin-top: 64px; text-align: center; color: #6b7280; font-size: 0.9rem;">
                { format!("© {} OS Web Sim — Built with Yew and Rust by ", Utc::now().year()) }
                <a
                target="_blank" rel="noopener noreferrer"
                href="https://github.com/MontyCoder0701/os-web-sim"
                >
                 {"MontyCoder0701"}
                </a>
            </footer>
        </div>
    }
}

fn render_gantt_chart(title: &str, logs: &[scheduling::ExecutionLog]) -> Html {
    const SCALE: f64 = 20.0;

    let base_time = logs
        .first()
        .map(|log| log.start_date_time)
        .unwrap_or(Utc.timestamp_opt(0, 0).single().unwrap());

    html! {
        <div style="margin-bottom: 48px;">
            <h2>{ title }</h2>
            <div style="display: flex; align-items: center; position: relative; height: 40px; background: #f3f4f6;">
                {
                    logs.iter().map(|log| {
                        let offset = (log.start_date_time - base_time).num_seconds() as f64 * SCALE;
                        let width = (log.end_date_time - log.start_date_time).num_seconds() as f64 * SCALE;
                        html! {
                            <div style={format!(
                                "position: absolute; left: {:.2}px; width: {:.2}px; height: 30px; background: #3b82f6; color: white; text-align: center; line-height: 30px; border-radius: 4px; border: 1px solid #1e3a8a;",
                                offset, width
                            )}>
                                { &log.pid }
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
