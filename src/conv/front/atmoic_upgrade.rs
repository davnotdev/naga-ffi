use super::*;

pub fn atmoic_upgrade_front_error_to_ffi(
    error: &naga::front::atomic_upgrade::Error,
) -> ffi::NagaAtomicUpgradeFrontError {
    match error {
        naga::front::atomic_upgrade::Error::Unsupported => {
            ffi::NagaAtomicUpgradeFrontError_NagaAtomicUpgradeFront_Unsupported
        }
        naga::front::atomic_upgrade::Error::UnexpectedEndOfIndices => {
            ffi::NagaAtomicUpgradeFrontError_NagaAtomicUpgradeFront_UnexpectedEndOfIndices
        }
        naga::front::atomic_upgrade::Error::GlobalInitUnsupported => {
            ffi::NagaAtomicUpgradeFrontError_NagaAtomicUpgradeFront_GlobalInitUnsupported
        }
        naga::front::atomic_upgrade::Error::GlobalVariableMissing => {
            ffi::NagaAtomicUpgradeFrontError_NagaAtomicUpgradeFront_GlobalVariableMissing
        }
        naga::front::atomic_upgrade::Error::CompareExchangeNonScalarBaseType => {
            ffi::NagaAtomicUpgradeFrontError_NagaAtomicUpgradeFront_CompareExchangeNonScalarBaseType
        }
    }
}
