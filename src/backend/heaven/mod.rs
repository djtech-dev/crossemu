pub mod heaven;

#[cfg(target_arch="x86")]
pub mod x86;
#[cfg(target_arch="x86_64")]
pub mod x86;
#[cfg(target_arch="aarch64")]
pub mod aarch64;

use self::heaven::Heaven;
/// Creates a new istance of the Heaven JIT compiler.
pub fn new_heaven() -> Heaven {
    Heaven::new()
}