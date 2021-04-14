use crate::spec::{LinkerFlavor, Target, TargetOptions};
use crate::abi::Endian;

pub fn target() -> Target {
    let base = super::linux_uclibc_base::opts();
    Target {
        llvm_target: "armeb-unknown-linux-uclibcgnueabi".to_string(),
        pointer_width: 32,
        data_layout: "E-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),

        options: TargetOptions {
            linker_flavor: LinkerFlavor::Gcc,
            endian: Endian::Big,
            features: "+strict-align,+v6,+soft-float".to_string(),
            cpu: "generic".to_string(),
            max_atomic_width: Some(32),
            unsupported_abis: super::arm_base::unsupported_abis(),
            mcount: "\u{1}__gnu_mcount_nc".to_string(),
            ..base
        },
    }
}
