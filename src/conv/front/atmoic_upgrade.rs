use super::*;

pub fn atmoic_upgrade_front_error_to_ffi(
    error: &naga::front::atomic_upgrade::Error,
) -> ffi::AtomicUpgradeFrontError {
    match error {
        naga::front::atomic_upgrade::Error::Unsupported => {
            ffi::AtomicUpgradeFrontError_AtomicUpgradeFront_Unsupported
        }
        naga::front::atomic_upgrade::Error::UnexpectedEndOfIndices => {
            ffi::AtomicUpgradeFrontError_AtomicUpgradeFront_UnexpectedEndOfIndices
        }
        naga::front::atomic_upgrade::Error::GlobalInitUnsupported => {
            ffi::AtomicUpgradeFrontError_AtomicUpgradeFront_GlobalInitUnsupported
        }
        naga::front::atomic_upgrade::Error::GlobalVariableMissing => {
            ffi::AtomicUpgradeFrontError_AtomicUpgradeFront_GlobalVariableMissing
        }
        naga::front::atomic_upgrade::Error::CompareExchangeNonScalarBaseType => {
            ffi::AtomicUpgradeFrontError_AtomicUpgradeFront_CompareExchangeNonScalarBaseType
        }
    }
}
