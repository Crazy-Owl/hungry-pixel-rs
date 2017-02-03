use std::env;
use std::path::PathBuf;

/// DEBUG const indicates whether we should look for resources and textures inside the
/// current dir subdirs or we should go up (since we're inside the debug build in repo)
#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

/// Returns a PathBuf for given resource, assembling it in accordance to DEBUG
pub fn get_resource_path(rname: &str) -> PathBuf {
    let exe = env::current_exe().unwrap();
    let exe_dir = exe.parent().unwrap();
    let resource_root = if DEBUG {
        exe_dir.parent().unwrap().parent().unwrap().join("resources")
    } else {
        exe_dir.join("resources")
    };

    resource_root.join(rname)
}
