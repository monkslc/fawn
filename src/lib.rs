#![cfg_attr(not(test), no_std)]
#![feature(asm)]

pub fn getpid() -> u32 {
    let mut pid: u32;

    unsafe {
        asm! {
            "mov rax, 39",
            "syscall",
            out("rax") pid,
        };
    };

    pid
}

mod tests {
    #[test]
    fn getpid() {
        let pid = super::getpid();
        assert_eq!(std::process::id(), pid);
    }
}
