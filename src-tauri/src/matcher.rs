use crate::grpc::proto::LibraryRule;
use std::env::consts::ARCH;
use std::env::consts::OS;

pub fn match_lib(rules: Vec<LibraryRule>) -> bool {
    if rules.is_empty() || rules.len() == 0 {
        return true;
    }

    let mut result: bool = false;

    rules.iter().for_each(|element| {
        if element.action == "allow" {
            result = true;
            match &element.os {
                Some(os) => {
                    result = is_os(&os.name) && is_arch(&os.arch);
                }
                None => {}
            }
        } else {
            // Disallow
            result = false;
            match &element.os {
                Some(os) => {
                    result = !is_os(&os.name) || !is_arch(&os.arch);
                }
                None => {}
            }
        }
    });
    result
}

fn is_os(os: &String) -> bool {
    if os == os_enum(OS) {
        return true;
    } else {
        return false;
    }
}

fn is_arch(arch: &Option<String>) -> bool {
    match arch {
        Some(arch) => {
            if arch == arch_enum(ARCH) {
                return true;
            } else {
                return false;
            }
        }
        None => return true,
    }
}

fn arch_enum(arch: &str) -> &str {
    match arch {
        "x86" => "x86",
        "x86_64" => "x86",
        "aarch64" => "arm64",
        "arm" => "arm64",
        _ => todo!(),
    }
}

fn os_enum(os: &str) -> &str {
    match os {
        "linux" => "linux",
        "windows" => "windows",
        "macos" => "osx",
        _ => todo!(),
    }
}
