//! [RISC-V Supervisor Binary Interface (SBI)](https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc)

pub mod base;
pub mod legacy;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct SBIReturn {
    error: SBIError,
    value: usize,
}

impl SBIReturn {
    fn unwrap(self) -> usize {
        assert_eq!(self.error, SBIError::Success);
        self.value
    }
}

/// The error type which is returned from SBI.
#[repr(isize)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum SBIError {
    Success = 0,
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
}

/// The type returned by SBI functions.
pub type SBIResult<T = ()> = Result<T, SBIError>;

impl From<SBIReturn> for SBIResult<usize> {
    fn from(ret: SBIReturn) -> Self {
        match ret.error {
            SBIError::Success => Ok(ret.value),
            err => Err(err),
        }
    }
}

#[inline(always)]
fn sbi_call(ext_id: usize, func_id: usize, arg0: usize, arg1: usize, arg2: usize) -> SBIReturn {
    let error;
    let value;
    unsafe {
        llvm_asm!(
            "ecall"
            : "={x10}" (error), "={x11}"(value)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x16}"(func_id), "{x17}" (ext_id)
            : "memory"
            : "volatile"
        );
    }
    SBIReturn { error, value }
}
