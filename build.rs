use std::io;

#[cfg(not(target_os = "windows"))]
fn main() -> io::Result<()> {
    println!("WASM Build script is being run");
    dbg!(cfg!(target_os = "unknown"));
    Ok(())
}

#[cfg(all(target_os = "windows"))]
fn main() -> io::Result<()> {
    println!("Windows Build Script is being Run");
    dbg!(cfg!(target_os = "unknown"));

    Ok(())
}
