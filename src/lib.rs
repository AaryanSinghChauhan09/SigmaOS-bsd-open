#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::fmt;

/// OpenBSD pledge promise variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Promise {
    Stdio,
    Rpath,
    Wpath,
    Cpath,
    Dpath,
    Tmppath,
    Inet,
    Flock,
    Unix,
    Dns,
    Getpw,
    Sendfd,
    Recvfd,
    Tape,
    Tty,
    Proc,
    Exec,
    ProtExec,
    Settime,
    Ps,
    Vminfo,
    Id,
    Pf,
    Route,
    Audio,
    Video,
    Bpf,
    Unveil,
    Error,
}

impl fmt::Display for Promise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Promise::Stdio => "stdio",
            Promise::Rpath => "rpath",
            Promise::Wpath => "wpath",
            Promise::Cpath => "cpath",
            Promise::Dpath => "dpath",
            Promise::Tmppath => "tmppath",
            Promise::Inet => "inet",
            Promise::Flock => "flock",
            Promise::Unix => "unix",
            Promise::Dns => "dns",
            Promise::Getpw => "getpw",
            Promise::Sendfd => "sendfd",
            Promise::Recvfd => "recvfd",
            Promise::Tape => "tape",
            Promise::Tty => "tty",
            Promise::Proc => "proc",
            Promise::Exec => "exec",
            Promise::ProtExec => "prot_exec",
            Promise::Settime => "settime",
            Promise::Ps => "ps",
            Promise::Vminfo => "vminfo",
            Promise::Id => "id",
            Promise::Pf => "pf",
            Promise::Route => "route",
            Promise::Audio => "audio",
            Promise::Video => "video",
            Promise::Bpf => "bpf",
            Promise::Unveil => "unveil",
            Promise::Error => "error",
        };
        write!(f, "{s}")
    }
}

/// A set of OpenBSD pledge promises.
#[derive(Debug, Clone)]
pub struct PledgeSet {
    pub promises: Vec<Promise>,
}

impl PledgeSet {
    /// Create a new `PledgeSet` from a vector of promises.
    pub fn new(promises: Vec<Promise>) -> Self {
        Self { promises }
    }

    /// Return the pledge(2) string — promises joined by spaces.
    pub fn pledge_string(&self) -> String {
        self.promises
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// An unveil(2) path entry with associated permissions.
#[derive(Debug, Clone)]
pub struct UnveilEntry {
    pub path: String,
    pub permissions: String,
}

impl UnveilEntry {
    /// Create a new `UnveilEntry`.
    pub fn new(path: &str, permissions: &str) -> Self {
        Self {
            path: path.to_string(),
            permissions: permissions.to_string(),
        }
    }

    /// Return a C-style `unveil(...)` statement string.
    pub fn unveil_statement(&self) -> String {
        format!("unveil(\"{}\", \"{}\");", self.path, self.permissions)
    }
}

/// A pf(4) firewall rule.
#[derive(Debug, Clone)]
pub struct PfRule {
    pub action: String,
    pub direction: String,
    pub proto: String,
    pub port: u16,
}

impl PfRule {
    /// Create a new `PfRule`.
    pub fn new(action: &str, direction: &str, proto: &str, port: u16) -> Self {
        Self {
            action: action.to_string(),
            direction: direction.to_string(),
            proto: proto.to_string(),
            port,
        }
    }

    /// Return the pf rule string, e.g. `pass in proto tcp to port 22`.
    pub fn pf_rule_string(&self) -> String {
        format!(
            "{} {} proto {} to port {}",
            self.action, self.direction, self.proto, self.port
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Arrange + Act + Assert: pledge_string joins promises with spaces
    #[test]
    fn test_pledge_string() {
        let set = PledgeSet::new(vec![Promise::Stdio, Promise::Inet, Promise::Dns]);
        let s = set.pledge_string();
        assert_eq!(s, "stdio inet dns");
    }

    // Arrange + Act + Assert: unveil_statement format
    #[test]
    fn test_unveil_statement() {
        let entry = UnveilEntry::new("/etc", "r");
        let stmt = entry.unveil_statement();
        assert_eq!(stmt, "unveil(\"/etc\", \"r\");");
    }

    // Arrange + Act + Assert: pf_rule_string format
    #[test]
    fn test_pf_rule() {
        let rule = PfRule::new("pass", "in", "tcp", 22);
        assert_eq!(rule.pf_rule_string(), "pass in proto tcp to port 22");
    }

    // Arrange + Act + Assert: Promise Display trait
    #[test]
    fn test_promise_display() {
        assert_eq!(Promise::Stdio.to_string(), "stdio");
        assert_eq!(Promise::ProtExec.to_string(), "prot_exec");
        assert_eq!(Promise::Dns.to_string(), "dns");
        assert_eq!(Promise::Pf.to_string(), "pf");
    }

    // Arrange + Act + Assert: PledgeSet with multiple promises
    #[test]
    fn test_pledge_set_multiple() {
        let set = PledgeSet::new(vec![
            Promise::Stdio,
            Promise::Rpath,
            Promise::Wpath,
            Promise::Exec,
        ]);
        let s = set.pledge_string();
        assert_eq!(s, "stdio rpath wpath exec");
    }

    // Arrange + Act + Assert: empty PledgeSet
    #[test]
    fn test_pledge_set_empty() {
        let set = PledgeSet::new(vec![]);
        assert_eq!(set.pledge_string(), "");
    }

    // Arrange + Act + Assert: single promise PledgeSet
    #[test]
    fn test_pledge_set_single() {
        let set = PledgeSet::new(vec![Promise::Tty]);
        assert_eq!(set.pledge_string(), "tty");
    }
}
