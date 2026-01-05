use zed_extension_api as zed;
use zed_extension_api::settings::LspSettings;

#[derive(Default)]
struct ProcessingExtension {
    cached_cli: Option<String>,
}

impl ProcessingExtension {
    fn configured_command(
        &self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::Command>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
        if let Some(binary) = settings.binary {
            if let Some(path) = binary.path {
                let mut command = zed::Command::new(path);
                command = command.envs(worktree.shell_env());
                if let Some(args) = binary.arguments {
                    command = command.args(args);
                }
                if let Some(env) = binary.env {
                    command = command.envs(env.into_iter());
                }
                return Ok(Some(command));
            }
        }
        Ok(None)
    }

    fn detect_cli(&mut self, worktree: &zed::Worktree) -> zed::Result<String> {
        if let Some(path) = self
            .cached_cli
            .as_ref()
            .filter(|path| !path.is_empty())
            .cloned()
        {
            return Ok(path);
        }

        let shell_env: Vec<(String, String)> = worktree.shell_env().into_iter().collect();
        let env_path = ["PROCESSING_PATH", "PROCESSING_BIN", "PROCESSING_CLI"]
            .iter()
            .find_map(|key| {
                shell_env
                    .iter()
                    .find(|(name, _)| name == key)
                    .map(|(_, value)| value.clone())
            });
        if let Some(path) = env_path {
            if !path.is_empty() {
                self.cached_cli = Some(path.clone());
                return Ok(path);
            }
        }

        if let Some(path) = worktree.which("processing") {
            self.cached_cli = Some(path.clone());
            return Ok(path);
        }

        let (os, _) = zed::current_platform();
        if matches!(os, zed::Os::Windows) {
            if let Some(path) = worktree.which("processing.exe") {
                self.cached_cli = Some(path.clone());
                return Ok(path);
            }
        }

        Err("Unable to locate the Processing executable. Add it to PATH, set a PROCESSING_PATH env var, or configure `lsp.processing-language-server.binary.path`.".to_string())
    }
}

impl zed::Extension for ProcessingExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let mut command =
            if let Some(command) = self.configured_command(language_server_id, worktree)? {
                command
            } else {
                let binary = self.detect_cli(worktree)?;
                zed::Command::new(binary).envs(worktree.shell_env())
            };

        command = command.arg("lsp");
        Ok(command)
    }
}

zed::register_extension!(ProcessingExtension);
