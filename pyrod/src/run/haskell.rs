use std::{ffi::OsString, io::Write, os::unix::prelude::OsStringExt};
use std::{os::unix::process::CommandExt, process::Command};
use std::{path::PathBuf, process::Stdio};

use super::RunError;

#[derive(Debug)]
pub struct HaskellRunner;

impl super::Runner for HaskellRunner {
    //haskell can also be run straight from source file
    //but we compile it anyway for speed
    #[tracing::instrument]
    fn compile(&self, code: String) -> Result<(), RunError> {
        std::fs::create_dir_all("/tmp")?;
        let path = PathBuf::from("/tmp/main.hs");
        std::fs::write(&path, code).map_err(RunError::from)?;
        tracing::debug!("Code written out to {path:?}");

        let child = Command::new("ghc") // where it's installed in alpine
            .current_dir("/tmp")
            .arg("main.hs")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .uid(111)
            .gid(111)
            .spawn()?;

        tracing::info!("GHC compile child process spawned");

        //collect output
        let output = child.wait_with_output()?;

        if output.status.success() {
            tracing::info!("Code compiled succesfully");
            Ok(())
        } else {
            tracing::error!("Code failed to compile");
            Err(RunError::CompileError(
                OsString::from_vec(output.stdout),
                OsString::from_vec(output.stderr),
            ))
        }
    }

    -rwxr-xr-x    1 root     root       32.1M Apr 19 07:29 cc1

    #[tracing::instrument(skip(self, stdin))]
    fn run(&self, stdin: String) -> Result<(OsString, OsString), RunError> {
        //spawn child process
        let mut child = Command::new("/tmp/main")
            .uid(111) //non-root uids
            .gid(111)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        tracing::debug!("Child process spawned");

        // pipe input
        child.stdin.take().unwrap().write_all(stdin.as_bytes())?;
        // collect output
        let output = child.wait_with_output()?;

        tracing::debug!("Output collected, process joined");

        Ok((
            OsString::from_vec(output.stdout),
            OsString::from_vec(output.stderr),
        ))
    }
}
