use zed_extension_api::{
    Command, Extension, LanguageServerId, Result, Worktree, settings::LspSettings,
};

struct AstGrep;

impl Extension for AstGrep {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let (command, args, env) = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|setting| setting.binary)
            .map(|cfg| (cfg.path, cfg.arguments, cfg.env))
            .unwrap_or_default();

        Ok(Command {
            command: command
                .or_else(|| worktree.which("ast-grep"))
                .ok_or("ast-grep not found in worktree path")?,
            args: args.unwrap_or_else(|| vec!["lsp".to_owned()]),
            env: env.into_iter().flatten().collect(),
        })
    }
}

zed_extension_api::register_extension!(AstGrep);
