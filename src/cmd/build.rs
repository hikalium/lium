// Copyright 2023 The ChromiumOS Authors
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

use anyhow::anyhow;
use anyhow::Result;
use argh::FromArgs;
use lium::chroot::Chroot;
use lium::repo::get_repo_dir;

#[derive(FromArgs, PartialEq, Debug)]
/// build package(s)
#[argh(subcommand, name = "build")]
pub struct Args {
    /// target cros repo dir
    #[argh(option)]
    repo: Option<String>,

    /// target board
    #[argh(option)]
    board: String,

    /// packages to build (or workon, for a full build)
    #[argh(positional)]
    packages: Vec<String>,

    /// if specified, skip update_chroot and setup_board
    #[argh(switch)]
    skip_setup: bool,

    /// if specified, do not stop working on packages already working on
    #[argh(switch)]
    keep_workon: bool,

    /// USE flags to be used, space separated
    #[argh(
        option,
        default = "String::from(\"chrome_internal -cros-debug pcserial\")"
    )]
    use_flags: String,

    /// do full build (build_packages + build_image)
    #[argh(switch)]
    full: bool,
}
pub fn run(args: &Args) -> Result<()> {
    let board = &args.board;
    let use_flags = &args.use_flags;
    let chroot = Chroot::new(&get_repo_dir(&args.repo)?)?;
    if !args.skip_setup {
        chroot.run_bash_script_in_chroot(
            "board_setup",
            &format!(
                r###"
setup_board --force --board={board}
./update_chroot
"###,
            ),
            None,
        )?;
    }
    if !args.keep_workon {
        chroot.run_bash_script_in_chroot(
            "stop_workon",
            &format!(
                r###"
cros-workon-{board} stop --all
"###
            ),
            None,
        )?;
    }
    if !args.packages.is_empty() {
        let package_list = args.packages.join(" ");
        chroot.run_bash_script_in_chroot(
            "start_workon",
            &format!(
                r###"
cros-workon-{board} start {package_list}
"###
            ),
            None,
        )?;
    }
    if args.full {
        eprintln!("building a full image...");
        // --accept_licenses is needed to resolve `masked by: Google-TOS license(s)` error.
        // https://www.chromium.org/chromium-os/licensing/building-a-distro/
        chroot.run_bash_script_in_chroot(
            "build_packages",
            &format!(
                r###"
export USE='{use_flags}'
build_packages --accept-licenses='*' --board={board} --withdev
build_image --board={board} --noenable_rootfs_verification test
"###
            ),
            None,
        )?;
        eprintln!("Succesfully built a test image!");
    } else if !args.packages.is_empty() {
        let package_list = args.packages.join(" ");
        eprintln!("Building {package_list}...");
        chroot.run_bash_script_in_chroot(
            "emerge_packages",
            &format!(
                r###"
export USE='{use_flags}'
emerge-{board} {package_list}
"###
            ),
            None,
        )?;
        eprintln!("Succesfully built {package_list}!");
    } else {
        return Err(anyhow!(
            "Please specify --full or --packages. `lium build --help` for more details."
        ));
    }
    Ok(())
}
