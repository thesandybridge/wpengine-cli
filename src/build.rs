fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or_else(|| std::io::ErrorKind::NotFound)?);

    let cmd = clap::Command::new("wpe")
        .arg(clap::arg!(-s --sites <SITES>))
        .arg(clap::arg!(-i --site <SITE>));

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("wpe.1"), buffer)?;

    Ok(())
}