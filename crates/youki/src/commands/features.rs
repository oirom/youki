//! Contains Functionality of `features` container command
use anyhow::Result;
use liboci_cli::Features;
use serde::{Serialize};
use serde_json;

/// Return the features list for a container
/// This subcommand was introduced in runc by
/// https://github.com/opencontainers/runc/pull/3296
/// It is documented here:
/// https://github.com/opencontainers/runtime-spec/blob/main/features-linux.md
#[derive(Serialize)]
struct FeaturesConfig {
    oci_version_min: String,
    oci_version_max: String,
    hooks: Vec<String>,
    mount_options: Vec<String>,
    linux: Linux
}

#[derive(Serialize)]
struct Linux {
    namespaces: Vec<String>,
    capabilities: Vec<String>,
    cgroup: Cgroup,
    seccomp: Seccomp,
    apparmor: Apparmor,
    selinux: Selinux,
    inet_rdt: InetRdt
}

#[derive(Serialize)]
struct Cgroup {
    v1: bool,
    v2: bool,
    systemd: bool,
    systemduser: bool,
    rdma: bool
}

#[derive(Serialize)]
struct Seccomp {
    enabled: bool,
    actions: Vec<String>,
    operators: Vec<String>,
    archs: Vec<String>,
    known_flags: Vec<String>,
    supported_flags: Vec<String>
}

#[derive(Serialize)]
struct Apparmor {
    enabled: bool
}

#[derive(Serialize)]
struct Selinux {
    enabled: bool
}

#[derive(Serialize)]
struct InetRdt {
    enabled: bool
}

pub(crate) fn features(_: Features) -> Result<()> {
    let features_config = FeaturesConfig {
        oci_version_min: "1.0.0".to_string(),
        oci_version_max: "1.1.0".to_string(),
        hooks: vec![
            "prestart".to_string(),
            "createRuntime".to_string(),
            "createContainer".to_string(),
            "startContainer".to_string(),
            "poststart".to_string(),
            "poststop".to_string()
        ],
        mount_options: vec![
            "async".to_string(),
            "atime".to_string(),
            "bind".to_string(),
            "defaults".to_string(),
            "dev".to_string(),
            "diratime".to_string(),
            "dirsync".to_string(),
            "exec".to_string(),
            "mand".to_string(),
            "noatime".to_string(),
            "nodiratime".to_string(),
            "nodev".to_string(),
            "noexec".to_string(),
            "nomand".to_string(),
            "norelatime".to_string(),
            "nostrictatime".to_string(),
            "nosuid".to_string(),
            "private".to_string(),
            "rbind".to_string(),
            "relatime".to_string(),
            "remount".to_string(),
            "ro".to_string(),
            "rprivate".to_string(),
            "rshared".to_string(),
            "rslave".to_string(),
            "runbindable".to_string(),
            "rw".to_string(),
            "suid".to_string(),
            "sync".to_string(),
            "shared".to_string(),
            "slave".to_string(),
            "strictatime".to_string(),
            "unbindable".to_string()
        ],
        linux: Linux {
            namespaces: vec![
                "cgroup".to_string(),
                "ipc".to_string(),
                "mount".to_string(),
                "network".to_string(),
                "pid".to_string(),
                "user".to_string(),
                "uts".to_string()
            ],
            capabilities: vec![
                "CAP_CHOWN".to_string()
            ],
            cgroup: Cgroup {
                v1: true,
                v2: true,
                systemd: true,
                systemduser: true,
                rdma: true
            },
            seccomp: Seccomp {
                enabled: true,
                actions: vec![
                    "SCMP_ACT_ALLOW".to_string(),
                    "SCMP_ACT_ERRNO".to_string(),
                    "SCMP_ACT_KILL".to_string(),
                    "SCMP_ACT_KILL_PROCESS".to_string(),
                    "SCMP_ACT_KILL_THREAD".to_string(),
                    "SCMP_ACT_LOG".to_string(),
                    "SCMP_ACT_NOTIFY".to_string(),
                    "SCMP_ACT_TRACE".to_string(),
                    "SCMP_ACT_TRAP".to_string()
                ],
                operators: vec![
                    "SCMP_CMP_EQ".to_string(),
                    "SCMP_CMP_GE".to_string(),
                    "SCMP_CMP_GT".to_string(),
                    "SCMP_CMP_LE".to_string(),
                    "SCMP_CMP_LT".to_string(),
                    "SCMP_CMP_MASKED_EQ".to_string(),
                    "SCMP_CMP_NE".to_string()
                ],
                archs: vec![
                    "SCMP_ARCH_AARCH64".to_string(),
                    "SCMP_ARCH_ARM".to_string(),
                    "SCMP_ARCH_MIPS".to_string(),
                    "SCMP_ARCH_MIPS64".to_string(),
                    "SCMP_ARCH_MIPS64N32".to_string(),
                    "SCMP_ARCH_MIPSEL".to_string(),
                    "SCMP_ARCH_MIPSEL64".to_string(),
                    "SCMP_ARCH_MIPSEL64N32".to_string(),
                    "SCMP_ARCH_PPC".to_string(),
                    "SCMP_ARCH_PPC64".to_string(),
                    "SCMP_ARCH_PPC64LE".to_string(),
                    "SCMP_ARCH_RISCV64".to_string(),
                    "SCMP_ARCH_S390".to_string(),
                    "SCMP_ARCH_S390X".to_string(),
                    "SCMP_ARCH_X32".to_string(),
                    "SCMP_ARCH_X86".to_string(),
                    "SCMP_ARCH_X86_64".to_string()
                ],
                known_flags: vec![
                    "SECCOMP_FILTER_FLAG_TSYNC".to_string(),
                    "SECCOMP_FILTER_FLAG_SPEC_ALLOW".to_string(),
                    "SECCOMP_FILTER_FLAG_LOG".to_string()
                ],
                supported_flags: vec![
                    "SECCOMP_FILTER_FLAG_TSYNC".to_string(),
                    "SECCOMP_FILTER_FLAG_SPEC_ALLOW".to_string(),
                    "SECCOMP_FILTER_FLAG_LOG".to_string()
                ]
            },
            apparmor: Apparmor {
                enabled: true
            },
            selinux: Selinux {
                enabled: true
            },
            inet_rdt: InetRdt {
                enabled: true
            }
        }
    };

    let serialized = serde_json::to_string_pretty(&features_config).unwrap();
    println!("{}", serialized);

    Ok(())
}
