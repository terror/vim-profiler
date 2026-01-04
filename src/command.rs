use super::*;

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

impl FromStr for Command {
  type Err = Error;

  fn from_str(value: &str) -> Result<Self> {
    match value {
      "vim" => Ok(Command::Vim),
      "nvim" | "neovim" => Ok(Command::Neovim),
      _ => Err(Error::InvalidCommand {
        cmd: value.to_owned(),
      }),
    }
  }
}
