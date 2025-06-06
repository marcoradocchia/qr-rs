use clap::CommandFactory;
use clap_complete::Shell;
use clap_mangen::Man;
use std::{env, fs};

const SUPPORTED_SHELLS: &[Shell] = &[
    Shell::Bash,
    Shell::Elvish,
    Shell::Fish,
    Shell::PowerShell,
    Shell::Zsh,
];

// Include Args struct.
include!("src/args.rs");

fn main() {
    // Create man & completions directories.
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Failed to retrieve out dir"));
    let man_dir = out_dir.join("man");
    let comp_dir = out_dir.join("completions");

    fs::create_dir_all(&man_dir).expect("Failed to create man directory");
    fs::create_dir_all(&comp_dir).expect("Failed to create completions directory");

    // Retrieve Args and set binary name.
    let mut cmd = Args::command();
    cmd.set_bin_name("qr");

    // Generate & write man page.
    let mut buffer: Vec<u8> = Vec::new();
    Man::new(cmd.clone()).render(&mut buffer).expect("Failed to generate man page");
    fs::write(man_dir.join("qr.1"), buffer).expect("Failed to write man page");

    // Generate shell completions.
    for shell in SUPPORTED_SHELLS {
        clap_complete::generate_to(*shell, &mut cmd, "qr", &comp_dir)
            .expect("Failed to generate completions");
    }
}
