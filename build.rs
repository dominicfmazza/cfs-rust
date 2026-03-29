extern crate bindgen;

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

// look up an environment variable and prefix its value to a given path
pub fn with_dir(dir: &str, path: &str) -> String {
    let dir_os_string = OsStr::new(dir);
    let path_os_string = OsStr::new(path);
    let full_path = PathBuf::from(env::var(dir_os_string).unwrap());
    String::from(full_path.join(path_os_string).to_str().unwrap())
}

// look up an environment variable and prefix its value to a given path,
// resulting in an include "-IPATH"
pub fn with_include(dir: &str, path: &str) -> String {
    let dir_os_string = OsStr::new(dir);
    let path_os_string = OsStr::new(path);
    let full_path = PathBuf::from(env::var(dir_os_string).unwrap());
    let joined_path = String::from(full_path.join(path_os_string).to_str().unwrap());
    let mut include = "-I".to_string();
    include.push_str(&joined_path);

    include
}

// look up an environment variable and turn its value into an include "-I"
pub fn as_include(dir: &str) -> String {
    let dir_os_string = OsStr::new(dir);
    let full_path = PathBuf::from(env::var(dir_os_string).unwrap());
    let mut include = "-I".to_string();
    include.push_str(&full_path.to_str().unwrap());
    println!("{}", include.as_str());

    include
}

fn main() {
    let bindings = bindgen::Builder::default()
        // cFE Core API Bindings
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_config_api_typedefs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_config.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_core_api_base_msgids.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_core_api_interface_cfg.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_endian.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_error.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_es_api_typedefs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_es.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_evs_api_typedefs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_evs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_fs_api_typedefs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_fs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_msg_api_typedefs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_msg.h"))
        .header(with_dir(
            "CFE_CORE_SRC",
            "inc/cfe_resourceid_api_typedefs.h",
        ))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_resourceid.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_sb_api_typedefs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_sb.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_tbl_api_typedefs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_tbl_filedef.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_tbl.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_time_api_typedefs.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_time.h"))
        .header(with_dir("CFE_CORE_SRC", "inc/cfe_version.h"))
        // PSP Includes
        // .header(with_dir("PSP_DIR", "fsw/inc/cfe_psp_configdata.h"))
        .header(with_dir("PSP_DIR", "fsw/inc/cfe_psp.h"))
        .header(with_dir("PSP_DIR", "fsw/pc-linux/inc/cfe_psp_config.h"))
        .header(with_dir("PSP_DIR", "fsw/pc-linux/inc/psp_version.h"))
        // OSAL Includes
        .header(with_dir("OSAL_DIR", "src/os/inc/common_types.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-binsem.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-bsp.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-clock.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-common.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-condvar.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-constants.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-countsem.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-dir.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-error.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-file.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-filesys.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-heap.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-idmap.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-macros.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-module.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-mutex.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-network.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-printf.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-queue.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-rwlock.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-select.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-shell.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-sockets.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-task.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-timebase.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-timer.h"))
        .header(with_dir("OSAL_DIR", "src/os/inc/osapi-version.h"))
        // Clang Arguments
        // Include Path CFE
        .clang_arg(with_include("CFE_CORE_SRC", "fsw/cfe-core/src/inc"))
        .clang_arg(with_include("CFE_CORE_SRC", "inc"))
        .clang_arg(with_include("CFE_CORE_SRC", "../config"))
        .clang_arg(as_include("CFS_MISSION_INC"))
        .clang_arg(with_include("CFS_MISSION", "build/inc"))
        .clang_arg(with_include("CFS_MISSION", "build/osal_public_api/inc"))
        .clang_arg(with_include("CFE_CORE_SRC", "fsw/platform_inc/cpu1"))
        .clang_arg(with_include("CFE_CORE_SRC", "fsw/platform_inc/cpu1"))
        .clang_arg(with_include(
            "CFS_MISSION",
            "cfe/modules/resourceid/fsw/inc/",
        ))
        .clang_arg(with_include("CFS_MISSION", "cfe/modules/es/fsw/inc/"))
        .clang_arg(with_include("CFS_MISSION", "cfe/modules/evs/fsw/inc/"))
        .clang_arg(with_include("CFS_MISSION", "cfe/modules/sb/fsw/inc/"))
        .clang_arg(with_include("CFS_MISSION", "cfe/modules/time/fsw/inc/"))
        .clang_arg(with_include("CFS_MISSION", "cfe/modules/fs/fsw/inc/"))
        .clang_arg(with_include("CFS_MISSION", "cfe/modules/msg/fsw/inc/"))
        .clang_arg(with_include(
            "CFS_MISSION",
            "build/native/default_cpu1/inc/",
        ))
        .clang_arg(with_include(
            "CFS_MISSION",
            "build/native/default_cpu1/psp/inc",
        ))
        // Include Path PSP
        .clang_arg(with_include("PSP_DIR", "fsw/inc"))
        .clang_arg(with_include("PSP_DIR", "fsw/pc-linux/inc"))
        // Include Path OSAL
        .clang_arg(with_include("OSAL_DIR", "src/os/inc"))
        .clang_arg(with_include("OSAL_DIR", "src/bsp/pc-linux/config"))
        // Define operating system for network_includes.h in CFE
        .clang_arg("-D_LINUX_OS_")
        // This should generate functions from macros with arguments, but
        // does not. I'm not sure what is wrong here.
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // write out bindings to the source directory, where they are re-exported.
    //bindings.write_to_file("src/bindings.rs").expect("Couldn't write bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
