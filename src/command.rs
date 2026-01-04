use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) enum Command {
  Neovim,
  Vim,
}

impl Display for Command {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Command::Vim => write!(f, "vim"),
      Command::Neovim => write!(f, "nvim"),
    }
  }
}

impl Command {
  pub fn parse(cmd: &str) -> Result<Self> {
    match cmd {
      "vim" => Ok(Command::Vim),
      "nvim" | "neovim" => Ok(Command::Neovim),
      _ => Err(Error::InvalidCommand {
        cmd: cmd.to_owned(),
      }),
    }
  }
}
