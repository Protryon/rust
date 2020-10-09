use crate::spec::{LinkerFlavor, Target, TargetOptions, TargetResult};

pub fn target() -> TargetResult {
    let base = super::linux_base::opts();
    Ok(Target {
        llvm_target: "armeb-unknown-linux-uclibcgnueabi".to_string(),
        target_endian: "big".to_string(),
        target_pointer_width: "32".to_string(),
        target_c_int_width: "32".to_string(),
        data_layout: "E-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".to_string(),
        arch: "arm".to_string(),
        target_os: "linux".to_string(),
        target_env: "uclibc".to_string(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Gcc,

        options: TargetOptions {
            features: "+strict-align,+v6,+soft-float".to_string(),
            cpu: "generic".to_string(),
            max_atomic_width: Some(32),
            unsupported_abis: super::arm_base::unsupported_abis(),
            target_mcount: "\u{1}__gnu_mcount_nc".to_string(),
            ..base
        },
    })
}
