pub struct BashWrapper;

impl BashWrapper {
    pub fn new() -> Self {
        BashWrapper
    }

    pub fn wrap(&self, cmd: &str) -> String {
        // Wrap command with bash -c and bwrap logic (or simple set -e for now as a fallback/wrapper)
        format!("bwrap --unshare-net --setenv PATH /bin:/usr/bin --ro-bind / / --dev-bind /dev /dev --proc /proc --tmpfs /tmp --tmpfs /var/tmp bash -c \"set -e; {}\"", cmd.replace("\"", "\\\""))
    }
}
