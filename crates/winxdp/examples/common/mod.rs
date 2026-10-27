use std::{env, ffi::CString, path::PathBuf};
use winxdp::{
    BindMode, Error, Result, Runtime, Socket,
    ebpf::{EbpfRuntime, Steering},
};
pub fn selected_rx(queues: u32) -> Result<Steering> {
    if env::var("WINXDP_ISOLATED").as_deref() != Ok("1") {
        return Err(Error::InvalidConfiguration(
            "set WINXDP_ISOLATED=1 only for a dedicated test NIC; redirects remove matching OS traffic",
        ));
    }
    let index = env::var("WINXDP_IFINDEX")
        .ok()
        .and_then(|s| s.parse().ok())
        .ok_or(Error::InvalidConfiguration("set WINXDP_IFINDEX"))?;
    let dll = PathBuf::from(
        env::var("WINXDP_EBPF_DLL")
            .map_err(|_| Error::InvalidConfiguration("set absolute WINXDP_EBPF_DLL"))?,
    );
    let artifact = PathBuf::from(
        env::var("WINXDP_EBPF_PROGRAM")
            .map_err(|_| Error::InvalidConfiguration("set absolute WINXDP_EBPF_PROGRAM (.sys)"))?,
    );
    let name =
        CString::new(env::var("WINXDP_EBPF_NAME").unwrap_or_else(|_| "winxdp_selected".into()))
            .map_err(|_| Error::InvalidConfiguration("invalid program name"))?;
    // The operator explicitly selects the installed trusted runtime DLL.
    let ebpf = unsafe { EbpfRuntime::load(&dll) }?;
    let runtime = Runtime::open()?;
    let mut sockets = Vec::new();
    for q in 0..queues {
        sockets.push(
            Socket::builder(&runtime)
                .interface(index)
                .queue(winxdp::QueueId(q))
                .receive(true)
                .transmit(false)
                .original_length(true)
                .bind_mode(BindMode::Generic)
                .build()?,
        );
    }
    Steering::redirect_selected(&ebpf, sockets, &artifact, &name)
}
