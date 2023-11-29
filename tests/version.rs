use git_version::{git_describe, git_version};
use std::env;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

// The derive implements <RuntimeOptions> == <RuntimeOptions> comparisons
#[derive(PartialEq)]
enum RuntimeOptions {
	Quiet,
	Debug,
}

//==============================================================================
// Auxiliary Functions

fn find_path_parent(top: &Path, name: &str) -> Option<PathBuf> {
	let mut odir = None;

	let osearch = Some(OsStr::new(name));

	for p in top.ancestors() {
		if odir.is_none() && p.is_dir() && p.file_name() == osearch {
			odir = Some(p);
		}
	}

	if let Some(d) = odir {
		odir = d.parent();
	}

	match odir {
		Some(d) => Some(PathBuf::from(d)),
		None => None,
	}
}

fn find_maindir(options: &[RuntimeOptions]) -> Result<PathBuf, Error> {
	let omdpth = match std::env::current_exe() {
		Ok(p) => Some(p),
		Err(_) => {
			eprintln!("Module Path unknown!");
			None
		}
	};
	let omdnm = match &omdpth {
		Some(p) => {
			match p.as_path().file_name() {
				Some(f) => Some(PathBuf::from(f)),
				None => {
					eprintln!("Module Name unknown!");
					None
				}
			} //match p.as_path().file_name()
		} //Some(p)
		None => None,
	};

	let owrkdir = match &omdpth {
		Some(pth) => match pth.as_path().parent() {
			Some(prnt) => Some(PathBuf::from(prnt)),
			None => None,
		},
		None => None,
	};

	let mut omndir = match &owrkdir {
		Some(wdir) => Some(PathBuf::from(wdir)),
		None => None,
	};

	match &omndir {
		Some(mdir) => {
			match find_path_parent(mdir.as_path(), "target") {
				Some(tdir) => omndir = Some(tdir),
				None => {
					if let Some(bdir) = find_path_parent(mdir.as_path(), "bin") {
						omndir = Some(bdir)
					}
				}
			} //match get_some_path_parent(&mdir, "target")
		}
		None => {}
	} //if let Some(mdir) = omndir

	if options.contains(&RuntimeOptions::Debug) && !options.contains(&RuntimeOptions::Quiet) {
		println!("md pth : '{:?}'", omdpth);
		println!("md nm : '{:?}'", omdnm);
		println!("mndir: '{:?}'", omndir);
		println!("wrkdir: '{:?}'", owrkdir);
	} //if options.contains(RuntimeOptions::Debug) && ! options.contains(RuntimeOptions::Quiet)

	match omndir {
		Some(d) => Ok(d),
		None => Err(Error::new(
			ErrorKind::NotFound,
			"Main Directory: could not extend Directory from Executable Path",
		)),
	} //match omndir
}

#[test]
fn git_describe_is_right() {
	let maindir = find_maindir(&[RuntimeOptions::Debug]).expect("maindir not found");

	println!("main directory to '{}'", maindir.display());

	assert!(env::set_current_dir(&maindir).is_ok());

	println!("new working directory to '{}'", env::current_dir().unwrap().display());

	println!("cd directory to '{:?}'; exists: {:?}", maindir.as_os_str(), maindir.exists());

	let pwd_cmd = std::process::Command::new("pwd").output();

	println!("pwd command: {:?}", pwd_cmd);
	println!("pwd result:\n{}", std::str::from_utf8(&pwd_cmd.unwrap().stdout).unwrap());

	let cd_cmd = std::process::Command::new("cd").arg(maindir).output();

	println!("cd command: {:?}", cd_cmd);
	println!("cd result:\n{}", std::str::from_utf8(&cd_cmd.unwrap().stdout).unwrap());

	let ls_cmd = std::process::Command::new("ls").arg("-lah").output();

	println!("ls command: {:?}", ls_cmd);
	println!("ls result:\n{}", std::str::from_utf8(&ls_cmd.unwrap().stdout).unwrap());

	let git_cmd = std::process::Command::new("git")
		.args(&["describe", "--always", "--dirty=-modified"])
		.output();

	println!("git command: {:?}", git_cmd);

	let git_out = git_cmd.expect("failed to execute git").stdout;

	let git_name = if git_out.len() > 0 {
		std::str::from_utf8(&git_out[..git_out.len() - 1]).expect("non-utf8 error?!")
	} else {
		""
	};

	println!("name: '{}'", git_name);
	println!("GIT_VERSION: '{}'", git_version!(args = ["--always", "--dirty=-modified"]));

	assert_eq!(git_version!(args = ["--always", "--dirty=-modified"]), git_name);
	assert_eq!(git_describe!("--always", "--dirty=-modified"), git_name);
	assert_eq!(git_version!(prefix = "[", suffix = "]"), format!("[{}]", git_name));
}

#[test]
fn not_git_repository() {
	let maindir = find_maindir(&[RuntimeOptions::Debug]).expect("maindir not found");
	let root = maindir.as_path().parent().expect("maindir has no parent");

	println!("root directory: '{}'", root.display());

	assert!(env::set_current_dir(&root).is_ok());

	println!("new working directory to '{}'", env::current_dir().unwrap().display());

	let git_cmd = std::process::Command::new("git")
		.args(&["describe", "--always", "--dirty=-modified"])
		.output();

	println!("git command: {:?}", git_cmd);

	let git_out = git_cmd.expect("failed to execute git").stdout;
	let git_name = if git_out.len() > 0 {
		std::str::from_utf8(&git_out[..git_out.len() - 1]).expect("non-utf8 error?!")
	} else {
		""
	};

	println!("name: '{}'", git_name);
	println!("GIT_VERSION: '{}'", git_version!(args = ["--always", "--dirty=-modified"]));

	assert_eq!(git_version!(args = ["--always", "--dirty=-modified"]), git_name);
	assert_eq!(git_describe!("--always", "--dirty=-modified"), git_name);
	assert_eq!(git_version!(prefix = "[", suffix = "]"), format!("[{}]", git_name));
}
