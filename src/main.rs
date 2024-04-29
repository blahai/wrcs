use std::process::Command;

fn main() {
    // The command you want to run
    let ydotool = "ydotool mousemove 50 50";
    // Execute the command
    let output = Command::new("sh")
        .arg("-c")
        .arg(ydotool)
        .output()
        .expect("Failed to execute command");

    // Print the output of the command
    if output.status.success() {
        println!("Command executed successfully");
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            print!("{}", stdout);
        }
    } else {
        println!("Command failed to execute");
        if let Ok(stderr) = String::from_utf8(output.stderr) {
            eprint!("{}", stderr);
        }
    }
}
