fn main() {
    // Hide the console on GUI startup for MS Windows
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
        println!("cargo:rustc-link-arg=/ENTRY:mainCRTStartup");
    } else {
        // Deal with BSD platforms linker issues (XCB and similar libraries)
        if cfg!(target_os = "openbsd") {
            println!("cargo:rustc-link-search=/usr/X11R6/lib");
        } else if cfg!(target_os = "freebsd") {
            println!("cargo:rustc-arg=-Wl");
            println!("cargo:rustc-link-search=/usr/local/lib");
        } else if cfg!(target_os = "netbsd") {
            println!("cargo:rustc-arg=-Wl");
            println!("cargo:rustc-link-search=/usr/pkg/lib");
            println!("cargo:rustc-link-arg=-rpath=/usr/pkg/lib");
        }
    }
}
