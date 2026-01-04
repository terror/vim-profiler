use super::*;

#[derive(Debug)]
pub(crate) struct Worker {
  command: Command,
  file: Option<PathBuf>,
  iter: i64,
  sys: bool,
}

impl Worker {
  /// Clean up the created `vim.log` file.
  fn clean() -> Result<()> {
    fs::remove_file("vim.log").context(error::RemoveLogSnafu)?;
    Ok(())
  }

  pub fn new(
    command: Command,
    iter: i64,
    sys: bool,
    file: Option<PathBuf>,
  ) -> Self {
    Self {
      command,
      file,
      iter,
      sys,
    }
  }

  /// Parse the contents of `vim.log`.
  ///
  /// 036.484  000.043  000.043: sourcing /path/to/plugin/file.vim
  ///                   ^^^^^^^                    ^^^^^^
  pub fn parse(&self) -> Result<HashMap<String, f64>> {
    let content = fs::read_to_string("vim.log").context(error::ReadLogSnafu)?;

    // In case the log contains windows-style path separators, they get replaced
    // with unix-style path separators. This saves us from a more complicated regex
    // pattern later on.
    let content = content.replace('\\', "/");

    if let Some(plugin_directory) = Self::plugin_directory(&content)? {
      let re = RegexBuilder::new(&format!(
        r"^\d+.\d+\s+\d+.\d+\s+(\d+.\d+): sourcing {plugin_directory}/([^/]+)/",
      ))
      .multi_line(true)
      .build()?;

      let mut plugins = HashMap::new();
      for capture in re.captures_iter(&content) {
        if let (Some(time), Some(plugin)) = (capture.get(1), capture.get(2)) {
          *plugins.entry(plugin.as_str().to_owned()).or_insert(0.0) +=
            time.as_str().parse::<f64>()?;
        }
      }

      if self.sys {
        for dir in &["/usr", "/usr/local"] {
          let re = RegexBuilder::new(&format!(
            r"^\d+.\d+\s+\d+.\d+\s+(\d+.\d+): sourcing {dir}/.+/([^/]+.vim)\n",
          ))
          .multi_line(true)
          .build()?;

          for capture in re.captures_iter(&content) {
            if let (Some(time), Some(plugin)) = (capture.get(1), capture.get(2))
            {
              *plugins.entry(plugin.as_str().to_owned()).or_insert(0.0) +=
                time.as_str().parse::<f64>()?;
            }
          }
        }
      }

      return Ok(plugins);
    }

    Err(error::Error::PluginDirectory)
  }

  /// Grabs the plugin directory from the `vim.log` files contents.
  ///
  /// In order to get the plugin directory we simply need to
  /// grab the most common directory that shows up in the
  /// `vim.log` file that is not a subdirectory of a system plugin
  /// directory e.g /usr or /usr/local.
  pub fn plugin_directory(content: &str) -> Result<Option<String>> {
    let re = RegexBuilder::new(
      r"^\d+.\d+\s+\d+.\d+\s+\d+.\d+: sourcing (.+?)/(?:[^/]+/)(?:autoload|ftdetect|plugin|syntax)/[^/]+",
    ).multi_line(true).build()?;

    let mut counts = HashMap::new();
    for capture in re.captures_iter(content) {
      if let Some(directory) = capture.get(1) {
        if !directory.as_str().starts_with("/usr") {
          *counts.entry(directory.as_str()).or_insert(0) += 1;
        }
      }
    }

    Ok(
      counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _v)| k.to_string()),
    )
  }

  /// Execute `vim --startuptime` in a child process.
  /// Upon executing this command a `vim.log` file should be parsed
  /// and relevant data should be returned as a map.
  ///
  /// This will accumulate each value over all iterations and finally
  /// return the map with average values.
  pub fn run(&self) -> Result<Vec<Plugin>> {
    let mut ret = HashMap::new();

    info!(
      "Executing `{} --startuptime` and parsing the log file {} time{}",
      self.command,
      self.iter,
      if self.iter > 1 { "s" } else { "" }
    );

    let file = self
      .file
      .as_ref()
      .unwrap_or(&PathBuf::new())
      .to_str()
      .unwrap_or("")
      .to_owned();

    for _ in 0..self.iter {
      let mut child = Cmd::new(format!("{}", self.command))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg(&file)
        .arg("--startuptime")
        .arg("vim.log")
        .arg("-f")
        .arg("-c")
        .arg("q")
        .spawn()
        .context(error::StartupTimeSnafu)?;

      child.wait()?;

      let plugins = self.parse()?;

      for (k, v) in &plugins {
        ret.entry(k.to_owned()).or_insert_with(Vec::new).push(*v);
      }

      Self::clean()?;
    }

    Ok(
      ret
        .iter()
        .map(|(k, v)| Plugin::new(k.to_owned(), v.to_owned()))
        .collect::<Vec<Plugin>>(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::prelude::*;

  #[test]
  fn plugin_directory() -> Result<()> {
    let content = r"
      042.203  011.313  010.704: sourcing /usr/local/Cellar/neovim/0.5.0/share/nvim/runtime/filetype.vim
      065.646  001.389  000.393: sourcing /usr/local/Cellar/neovim/0.5.0/share/nvim/runtime/syntax/syntax.vim
      038.356  000.029  000.029: sourcing /Users/.vim/plugged/vim-prettier/ftdetect/graphql.vim
      039.955  000.028  000.028: sourcing /Users/.vim/plugged/vim-polyglot/ftdetect/polyglot.vim
      040.327  000.058  000.058: sourcing /Users/.vim/plugged/vim-markdown/ftdetect/markdown.vim
      040.530  000.048  000.048: sourcing /Users/.vim/plugged/rust.vim/ftdetect/rust.vim
    ";

    assert_eq!(
      Worker::plugin_directory(&dedent(content))?.unwrap(),
      "/Users/.vim/plugged"
    );

    Ok(())
  }

  #[test]
  fn plugin_directory_empty_content() -> Result<()> {
    assert!(Worker::plugin_directory("")?.is_none());
    Ok(())
  }

  #[test]
  fn parse() -> Result<()> {
    let content = r"
      038.356  000.029  000.029: sourcing /Users/.vim/plugged/vim-prettier/ftdetect/graphql.vim
      039.955  000.028  000.028: sourcing /Users/.vim/plugged/vim-polyglot/ftdetect/polyglot.vim
      040.327  000.058  000.058: sourcing /Users/.vim/plugged/vim-markdown/ftdetect/markdown.vim
      040.530  000.048  000.048: sourcing /Users/.vim/plugged/rust.vim/ftdetect/rust.vim
    ";

    let cases = vec![
      ("vim-prettier", 0.029),
      ("vim-polyglot", 0.028),
      ("vim-markdown", 0.058),
      ("rust.vim", 0.048),
    ];

    let mut file = fs::File::create("vim.log")?;
    file.write_all(dedent(content).as_bytes())?;

    let data = Worker::new(Command::Vim, 1, false, None).parse()?;
    for (key, value) in cases {
      assert!(approx_eq!(f64, data[key], value, ulps = 2));
    }

    fs::remove_file("vim.log")?;

    Ok(())
  }
}
