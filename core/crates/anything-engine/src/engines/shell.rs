use std::{
    env::temp_dir,
    fs::{remove_file, File},
    io::BufReader,
    process::{Command, Stdio},
};

extern crate fs_extra;

use std::io::Read;

use anything_graph::flow::action::ShellAction;
use fs_extra::{copy_items, dir};
use miette::IntoDiagnostic;
use serde::{Deserialize, Serialize};

use crate::error::{EngineError, EngineResult};

use super::{Engine, ExecEnv, Process, ProcessState};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ShellEngine {
    pub config: ShellAction,
    pub process: Option<Process>,
}

impl ShellEngine {
    pub fn new(config: ShellAction) -> Self {
        Self {
            config,
            process: None,
        }
    }
}

impl ShellEngine {
    pub fn clean(&self) -> EngineResult<()> {
        if self.process.is_none() {
            tracing::error!("Cannot clean a process that has not run");
            return Err(EngineError::ShellProcessHasNotRunError);
        }
        let process = self.process.as_ref().unwrap();
        let base_dir = process.env.directory.clone();
        let uuid = process.uuid.clone();

        let stdout_path = format!("{}/{}_stdout", base_dir.to_string_lossy(), uuid);
        let stderr_path = format!("{}/{}_stderr", base_dir.to_string_lossy(), uuid);

        remove_file(stdout_path).into_diagnostic()?;
        remove_file(stderr_path).into_diagnostic()?;
        Ok(())
    }

    fn read_status(&mut self) -> EngineResult<()> {
        if self.process.is_none() {
            tracing::error!("Cannot clean a process that has not run");
            return Err(EngineError::ShellProcessHasNotRunError);
        }

        let process = self.process.as_mut().unwrap();
        let stdout_path = format!(
            "{}/{}_stdout",
            process.env.directory.to_string_lossy(),
            process.uuid
        );
        let stderr_path = format!(
            "{}/{}_stderr",
            process.env.directory.to_string_lossy(),
            process.uuid
        );

        let f = File::open(stdout_path).into_diagnostic()?;
        let mut buf_reader = BufReader::new(f);
        let mut stdout = String::new();
        buf_reader.read_to_string(&mut stdout).into_diagnostic()?;

        let f = File::open(stderr_path).into_diagnostic()?;
        let mut buf_reader = BufReader::new(f);
        let mut stderr = String::new();
        buf_reader.read_to_string(&mut stderr).into_diagnostic()?;

        let state = ProcessState {
            status: process.state.status.clone(),
            stdin: process.state.stdin.clone(),
            stderr: Some(stderr),
            stdout: Some(stdout),
        };

        // process.state = state;
        let process = process.clone();
        self.process = Some(Process {
            state: state,
            ..process
        });

        Ok(())
    }
}

impl Engine for ShellEngine {
    // TODO: continue run?
    fn run(&mut self) -> EngineResult<()> {
        self.validate()?;

        let uuid = uuid::Uuid::new_v4();

        let config = self.config.clone();

        let executor = match config.executor {
            None => "/bin/sh".to_string(),
            Some(v) => v,
        }
        .clone();

        let base_dir = temp_dir();
        let base_dir_str = base_dir.to_string_lossy();
        match config.cwd {
            None => {}
            Some(v) => {
                // Copy files from the cwd
                let from_paths = vec![v];
                let options = dir::CopyOptions::new();
                copy_items(&from_paths, base_dir.clone(), &options)?;
            }
        };

        let stdout_path = format!("{}/{}_stdout", &base_dir_str, uuid);
        let stderr_path = format!("{}/{}_stderr", &base_dir_str, uuid);
        let stdout = Stdio::from(File::create(&stdout_path).into_diagnostic()?);
        let stderr = Stdio::from(File::create(&stderr_path).into_diagnostic()?);

        let args = config.args;

        let mut child = Command::new(executor);
        let child = child.arg("-c");
        let mut child = child.arg(config.command);

        for arg in args {
            child = child.arg(arg);
        }

        let child = child
            .current_dir(&base_dir)
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .into_diagnostic()?;

        let child_pid = child.id();
        let output = child.wait_with_output().into_diagnostic()?;
        let status = ProcessState::from(&output).status;

        let state = ProcessState {
            // status:
            status,
            stdin: None, // TODO: fill this in
            stdout: Some(stdout_path),
            stderr: Some(stderr_path),
        };

        let env = ExecEnv {
            directory: base_dir,
            attached: false,
            pid: Some(child_pid),
        };

        let process = Process { uuid, env, state };
        self.process = Some(process);

        let _ = self.read_status();
        let _ = self.clean();

        Ok(())
    }

    fn validate(&self) -> EngineResult<bool> {
        // Validate command is greater than ""
        if self.config.command.is_empty() {
            return Err(EngineError::ShellError("command is empty".to_string()));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shell_executes_with_simple() -> anyhow::Result<()> {
        let config = ShellAction {
            executor: None,
            command: "echo 'ducks'".to_string(),
            args: vec![],
            cwd: None,
        };

        let mut action = ShellEngine {
            config,
            process: None,
        };

        action.run()?;
        assert!(action.process.is_some());
        let process = action.process.unwrap();
        assert!(process.state.stdout.is_some());
        let stdout = process.state.stdout.unwrap();
        assert_eq!(stdout, "ducks\n");

        Ok(())
    }

    #[tokio::test]
    async fn test_shell_errors_with_no_command() -> anyhow::Result<()> {
        let mut action = ShellEngine {
            config: ShellAction {
                executor: None,
                command: "".to_string(),
                args: vec![],
                cwd: None,
            },
            process: None,
        };

        let res = action.run();
        assert!(res.is_err());
        Ok(())
    }
}