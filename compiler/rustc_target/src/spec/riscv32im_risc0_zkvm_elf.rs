// use crate::spec::cvs;
use crate::spec::{LinkerFlavor, LldFlavor, PanicStrategy, RelocModel};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".into(),
        llvm_target: "riscv32".into(),
        pointer_width: 32,
        arch: "riscv32".into(),

        options: TargetOptions {
            // families: cvs!["unix"],
            os: "zkvm".into(),
            // env: "newlib".into(),
            vendor: "risc0".into(),
            linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
            linker: Some("rust-lld".into()),
            cpu: "generic-rv32".into(),

            // TODO
            max_atomic_width: Some(32),
            atomic_cas: true,

            features: "+m".into(),
            executables: true,
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            emit_debug_gdb_scripts: false,
            eh_frame_header: false,

            // ZKVM doesn't have atomics yet, so tell LLVM that we're in a single
            // threaded model which will legalize atomics to normal operations.
            singlethread: true,

            // no dynamic linking, no need for default visibility!
            default_hidden_visibility: true,
            ..Default::default()
        },
    }
}
