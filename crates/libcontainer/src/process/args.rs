use libcgroups::common::CgroupConfig;
use oci_spec::runtime::Spec;
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::prelude::{RawFd, OwnedFd};
use std::path::PathBuf;
use std::rc::Rc;
use std::io::Result;

use crate::container::Container;
use crate::notify_socket::NotifyListener;
use crate::syscall::syscall::SyscallType;
use crate::user_ns::UserNamespaceConfig;
use crate::workload::Executor;

#[derive(Debug)]
pub struct MyOwnedFd(pub OwnedFd);

impl Clone for MyOwnedFd {
    fn clone(&self) -> Self {
        Self(unsafe { OwnedFd::from_raw_fd(self.0.as_raw_fd()) })
    }
}

impl MyOwnedFd {
    fn try_clone(&self) -> Result<Self> {
        Ok(Self(OwnedFd::try_clone(&self.0)?))
    }
}

#[derive(Debug, Clone)]
pub enum ContainerType {
    InitContainer,
    TenantContainer { exec_notify_fd: MyOwnedFd },
}

#[derive(Clone)]
pub struct ContainerArgs {
    /// Indicates if an init or a tenant container should be created
    pub container_type: ContainerType,
    /// Interface to operating system primitives
    pub syscall: SyscallType,
    /// OCI compliant runtime spec
    pub spec: Rc<Spec>,
    /// Root filesystem of the container
    pub rootfs: PathBuf,
    /// Socket to communicate the file descriptor of the ptty
    pub console_socket: Option<MyOwnedFd>,
    /// The Unix Domain Socket to communicate container start
    pub notify_listener: NotifyListener,
    /// File descriptors preserved/passed to the container init process.
    pub preserve_fds: i32,
    /// Container state
    pub container: Option<Container>,
    /// Options for new namespace creation
    pub user_ns_config: Option<UserNamespaceConfig>,
    /// Cgroup Manager Config
    pub cgroup_config: CgroupConfig,
    /// If the container is to be run in detached mode
    pub detached: bool,
    /// Manage the functions that actually run on the container
    pub executor: Box<dyn Executor>,
}
