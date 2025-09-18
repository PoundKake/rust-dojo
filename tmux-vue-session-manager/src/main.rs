use std::env;
use std::process::Command;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: tmux-vue-session-manager <path> <session_prefix>");
        std::process::exit(1);
    }

    let path = &args[1];
    let session_prefix = &args[2];
    let session_name = format!("{}_session", session_prefix);

    // Check if session exists
    let session_exists = Command::new("tmux")
        .arg("list-sessions")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).contains(&session_name))
        .unwrap_or(false);

    if !session_exists {
        // Create new tmux session
        Command::new("tmux")
            .args(["new-session", "-d", "-s", &session_name, "-c", path])
            .status()
            .expect("Failed to create tmux session");

        // Split windows
        Command::new("tmux")
            .args(["split-window", "-h", "-c", path])
            .status()
            .expect("Failed to split window horizontally");

        Command::new("tmux")
            .args(["split-window", "-v", "-c", path])
            .status()
            .expect("Failed to split window vertically");

        // Send commands to each pane
        Command::new("tmux")
            .args([
                "send-keys",
                "-t",
                &format!("{}:0.0", session_name),
                "zsh",
                "C-m",
                "npm run clean",
                "C-m",
            ])
            .status()
            .expect("Failed to send keys to pane 0");

        Command::new("tmux")
            .args([
                "send-keys",
                "-t",
                &format!("{}:0.1", session_name),
                "zsh",
                "C-m",
                "npm run dev",
                "C-m",
            ])
            .status()
            .expect("Failed to send keys to pane 1");

        Command::new("tmux")
            .args([
                "send-keys",
                "-t",
                &format!("{}:0.2", session_name),
                "zsh",
                "C-m",
                "npm run test:unit",
                "C-m",
            ])
            .status()
            .expect("Failed to send keys to pane 2");

        // Focus on working pane
        Command::new("tmux")
            .args(["select-pane", "-t", &format!("{}:0.0", session_name)])
            .status()
            .expect("Failed to select pane");
    }

    // Attach to the session
    Command::new("tmux")
        .args(["attach-session", "-t", &session_name])
        .status()
        .expect("Failed to attach to tmux session");
}
