//! Syma build system — cargo-xtask
//!
//! Usage: `cargo xtask <command> [options]`
//!
//! Commands:
//!   build            Build the syma binary
//!   install          Build and install to $SYMA_HOME
//!   dist             Create a distributable archive
//!   test             Run all tests
//!   lint             Run fmt check + clippy
//!   clean            Clean build artifacts
//!   setup-sysfiles   Create SystemFiles skeleton in $SYMA_HOME

use anyhow::{Context, Result, bail};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("build") => cmd_build(),
        Some("install") => cmd_install(),
        Some("dist") => cmd_dist(),
        Some("test") => cmd_test(),
        Some("lint") => cmd_lint(),
        Some("clean") => cmd_clean(),
        Some("setup-sysfiles") => cmd_setup_sysfiles(),
        _ => {
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!(
        "Syma build system — cargo-xtask\n\
         \n\
         Usage: cargo xtask <command>\n\
         \n\
         Commands:\n\
         \x20  build [--release] [--features <f>]  Build the syma binary\n\
         \x20  install [--release]                  Build and install to $SYMA_HOME\n\
         \x20  dist                                 Create a distributable archive\n\
         \x20  test                                 Run all tests\n\
         \x20  lint                                 Run fmt check + clippy\n\
         \x20  clean                                Clean build artifacts\n\
         \x20  setup-sysfiles                       Create SystemFiles skeleton in $SYMA_HOME"
    );
}

// ── Helpers ─────────────────────────────────────────────────────────────────

fn project_root() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    // xtask/ is one level below the workspace root
    Ok(manifest_dir
        .parent()
        .context("xtask must be a direct child of the workspace root")?
        .to_path_buf())
}

fn syma_home() -> PathBuf {
    if let Ok(home) = env::var("SYMA_HOME") {
        PathBuf::from(home)
    } else if let Some(home) = dirs_home() {
        home.join(".syma")
    } else {
        PathBuf::from(".syma")
    }
}

fn dirs_home() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

fn cargo() -> Command {
    Command::new(env::var("CARGO").unwrap_or_else(|_| "cargo".to_string()))
}

fn run_cargo(args: &[&str], root: &Path) -> Result<()> {
    let status = cargo()
        .args(args)
        .current_dir(root)
        .status()
        .with_context(|| format!("failed to run: cargo {}", args.join(" ")))?;
    if !status.success() {
        bail!("cargo {} exited with {}", args.join(" "), status);
    }
    Ok(())
}

// ── build ───────────────────────────────────────────────────────────────────

fn cmd_build() -> Result<()> {
    let root = project_root()?;
    let args: Vec<String> = env::args().skip(2).collect();

    let mut cargo_args = vec!["build", "--package", "syma"];
    let mut release = false;
    let mut features = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--release" => release = true,
            "--features" => {
                i += 1;
                if let Some(f) = args.get(i) {
                    features.push(f.clone());
                }
            }
            other => bail!("unknown build option: {other}"),
        }
        i += 1;
    }

    if release {
        cargo_args.push("--release");
    }
    if !features.is_empty() {
        cargo_args.push("--features");
        // We need to leak the string to get a &'static str for the args slice.
        // This is fine for a build tool that runs once.
        let features_str = features.join(",");
        cargo_args.push(leak_str(&features_str));
    }

    println!("   Building syma...");
    run_cargo(&cargo_args, &root)?;
    println!("   Done.");
    Ok(())
}

/// Leak a string to get a `&'static str`. Acceptable for short-lived xtask processes.
fn leak_str(s: &String) -> &'static str {
    // SAFETY: xtask is a short-lived process, leaking a few strings is fine.
    unsafe { &*(s as *const String).cast::<&'static str>() }
}

// ── install ─────────────────────────────────────────────────────────────────

fn cmd_install() -> Result<()> {
    let root = project_root()?;
    let args: Vec<String> = env::args().skip(2).collect();
    let release = args.iter().any(|a| a == "--release");

    // Build first
    let mut build_args = vec![
        "build".to_string(),
        "--package".to_string(),
        "syma".to_string(),
    ];
    if release {
        build_args.push("--release".to_string());
    }
    let build_args_refs: Vec<&str> = build_args.iter().map(|s| s.as_str()).collect();
    println!("   Building syma...");
    run_cargo(&build_args_refs, &root)?;

    // Locate the built binary
    let profile = if release { "release" } else { "debug" };
    let binary = root.join("target").join(profile).join("syma");
    if !binary.exists() {
        bail!("binary not found at {}", binary.display());
    }

    // Create $SYMA_HOME layout
    let home = syma_home();
    println!("   Installing to {}...", home.display());

    let bin_dir = home.join("bin");
    fs::create_dir_all(&bin_dir)?;

    let dest = bin_dir.join("syma");
    fs::copy(&binary, &dest)
        .with_context(|| format!("failed to copy binary to {}", dest.display()))?;

    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dest, fs::Permissions::from_mode(0o755))?;
    }

    // Create SystemFiles skeleton
    create_sysfiles_skeleton(&home)?;

    println!("   Installed: {}", dest.display());
    println!();
    println!("   Add to PATH:");
    println!("     export PATH=\"{}/bin:$PATH\"", home.display());
    Ok(())
}

// ── dist ────────────────────────────────────────────────────────────────────

fn cmd_dist() -> Result<()> {
    let root = project_root()?;

    // Always build release for distribution
    println!("   Building release...");
    run_cargo(&["build", "--package", "syma", "--release"], &root)?;

    let binary = root.join("target").join("release").join("syma");
    if !binary.exists() {
        bail!("release binary not found at {}", binary.display());
    }

    // Create staging directory
    let dist_dir = root.join("target").join("dist");
    let version = "0.1.0"; // TODO: read from Cargo.toml
    let stage = dist_dir.join(format!("syma-{version}"));
    if stage.exists() {
        fs::remove_dir_all(&stage)?;
    }
    fs::create_dir_all(&stage)?;

    // Copy binary
    let bin_dir = stage.join("bin");
    fs::create_dir_all(&bin_dir)?;
    fs::copy(&binary, bin_dir.join("syma"))?;

    // Create SystemFiles skeleton in staging
    create_sysfiles_skeleton(&stage)?;

    // Create archive
    let archive_name = format!("syma-{version}.tar.gz");
    let archive_path = dist_dir.join(&archive_name);

    println!("   Creating {}...", archive_name);
    let status = Command::new("tar")
        .args([
            "-czf",
            archive_path.to_str().unwrap(),
            "-C",
            dist_dir.to_str().unwrap(),
            &format!("syma-{version}"),
        ])
        .status()?;

    if !status.success() {
        bail!("tar failed");
    }

    // Clean up staging
    fs::remove_dir_all(&stage)?;

    println!("   Created: {}", archive_path.display());
    Ok(())
}

// ── test ────────────────────────────────────────────────────────────────────

fn cmd_test() -> Result<()> {
    let root = project_root()?;
    println!("   Running tests...");
    run_cargo(&["test", "--locked", "--workspace"], &root)?;
    Ok(())
}

// ── lint ────────────────────────────────────────────────────────────────────

fn cmd_lint() -> Result<()> {
    let root = project_root()?;
    println!("   Checking formatting...");
    run_cargo(&["fmt", "--check"], &root)?;
    println!("   Running clippy...");
    run_cargo(
        &["clippy", "--locked", "--workspace", "--", "-D", "warnings"],
        &root,
    )?;
    Ok(())
}

// ── clean ───────────────────────────────────────────────────────────────────

fn cmd_clean() -> Result<()> {
    let root = project_root()?;
    println!("   Cleaning build artifacts...");
    run_cargo(&["clean"], &root)?;

    let dist_dir = root.join("target").join("dist");
    if dist_dir.exists() {
        fs::remove_dir_all(&dist_dir)?;
        println!("   Removed target/dist/");
    }

    println!("   Done.");
    Ok(())
}

// ── setup-sysfiles ──────────────────────────────────────────────────────────

fn cmd_setup_sysfiles() -> Result<()> {
    let home = syma_home();
    println!("   Creating SystemFiles in {}...", home.display());
    create_sysfiles_skeleton(&home)?;
    println!("   Done.");
    Ok(())
}

// ── SystemFiles skeleton ────────────────────────────────────────────────────

fn create_sysfiles_skeleton(base: &Path) -> Result<()> {
    let dirs = [
        "bin",
        "SystemFiles/Kernel",
        "SystemFiles/Data/Rubi",
        "SystemFiles/Data/Chemistry",
        "SystemFiles/Data/Physics",
        "SystemFiles/Formats",
        "SystemFiles/Links",
        "Packages",
        "Extensions",
    ];

    for dir in &dirs {
        fs::create_dir_all(base.join(dir))?;
    }

    // Generate init.toml skeleton if it doesn't exist
    let init_toml = base.join("SystemFiles/Kernel/init.toml");
    if !init_toml.exists() {
        fs::write(
            &init_toml,
            r#"# Syma SystemFiles module registry
# This file is read at startup by the PackageManager.
# Each [[module]] entry declares a loadable builtin module.

# Example:
# [[module]]
# name = "arithmetic"
# file = "arithmetic.syma"
# symbols = ["Plus", "Times", "Power", "Divide", "Minus", "Abs"]
# builtin = true
"#,
        )?;
    }

    Ok(())
}
