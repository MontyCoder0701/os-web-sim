use chrono::{Duration, Utc};
use schedules::{FirstComeFirstServed, Process, RoundRobin, Scheduler, ShortestJobFirst};
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

    let rr = RoundRobin {
        quantum: Duration::seconds(2),
    };
    let scsf = FirstComeFirstServed;
    let sjf = ShortestJobFirst;

    let rr_logs = rr.schedule(processes.clone());
    let fcfs_logs = scsf.schedule(processes.clone());
    let sjf_logs = sjf.schedule(processes.clone());

    html! {
        <div>
            <div>
                <h1>{ "Round Robin" }</h1>
                <table border="1">
                    <tr>
                        <th>{ "PID" }</th>
                        <th>{ "Start" }</th>
                        <th>{ "End" }</th>
                    </tr>
                    {
                        rr_logs.iter().map(|log| {
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

            <div>
                <h1>{ "First Come First Served" }</h1>
                <table border="1">
                    <tr>
                        <th>{ "PID" }</th>
                        <th>{ "Start" }</th>
                        <th>{ "End" }</th>
                    </tr>
                    {
                        fcfs_logs.iter().map(|log| {
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

            <div>
                <h1>{ "Shortest Job First" }</h1>
                <table border="1">
                    <tr>
                        <th>{ "PID" }</th>
                        <th>{ "Start" }</th>
                        <th>{ "End" }</th>
                    </tr>
                    {
                        sjf_logs.iter().map(|log| {
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
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
