use std::process::Command;

fn main() {
    // Block incoming SSH connections from IP address 1.2.3.4
    Command::new("iptables")
            .args(&["-A", "INPUT", "-p", "tcp", "--dport", "22", "-s", "1.2.3.4", "-j", "DROP"])
            .status()
            .expect("failed to block SSH from 1.2.3.4");

    // Allow incoming HTTP connections
    Command::new("iptables")
            .args(&["-A", "INPUT", "-p", "tcp", "--dport", "80", "-j", "ACCEPT"])
            .status()
            .expect("failed to allow HTTP");

    // Block all other incoming traffic by default
    Command::new("iptables")
            .args(&["-P", "INPUT", "DROP"])
            .status()
            .expect("failed to set default policy to DROP");
}
