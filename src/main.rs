use chrono::{Duration, Utc};
use schedules::{Process, round_robin};
use yew::prelude::*;

mod schedules;

#[function_component]
fn App() -> Html {
    let processes = vec![
        Process {
            id: "P1".to_string(),
            arrival_date_time: Utc::now(),
            burst_duration: Duration::seconds(5),
        },
        Process {
            id: "P2".to_string(),
            arrival_date_time: Utc::now(),
            burst_duration: Duration::seconds(3),
        },
        Process {
            id: "P3".to_string(),
            arrival_date_time: Utc::now(),
            burst_duration: Duration::seconds(6),
        },
    ];

    let quantum = Duration::seconds(2);
    let logs = round_robin(processes, quantum);

    html! {
        <div>
            <h1>{ "Round Robin" }</h1>
            <table border="1">
                <tr>
                    <th>{ "PID" }</th>
                    <th>{ "Start" }</th>
                    <th>{ "End" }</th>
                </tr>
                {
                    logs.iter().map(|log| {
                        html! {
                            <tr>
                                <td>{ &log.pid }</td>
                                <td>{ log.start_date_time.to_rfc2822() }</td>
                                <td>{ log.end_date_time.to_rfc2822() }</td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </table>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
