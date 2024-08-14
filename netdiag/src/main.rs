use tokio::process::Command as TokioCommand;

#[tokio::main]
async fn main() {
    let ips = vec![
        "159.196.255.95",
        "180.150.0.101",
        "180.150.0.106",
        "180.150.0.107",
        "180.150.0.112",
        "180.150.0.22",
        "180.150.0.64",
        "180.150.0.99",
        "180.150.2.1",
        "180.150.2.4",

    ];

    let mut tasks = Vec::new();

    for ip in ips {
        let ip = ip.to_string();
        let task = tokio::spawn(async move {
            let output = TokioCommand::new("ping")
                .arg("-n 1") // For Linux/Mac; use "-n 4" for Windows
                .arg(&ip)
                .output()
                .await
                .expect("Failed to execute command");

            if !output.stdout.is_empty() {
               println!("Ping output for {}:\n\n{}\n", ip, String::from_utf8_lossy(&output.stdout)); 
            }

            if !output.stderr.is_empty() {
                eprintln!("Error pinging {}", ip);
            }
        });

        tasks.push(task);
    }

    // Await all tasks to complete
    for task in tasks {
        task.await.expect("Task failed");
    }
}

