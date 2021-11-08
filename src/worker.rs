use crate::common::*;

#[derive(Debug)]
pub struct Worker {
  command: Command,
  iter:    i64,
  sys:     bool,
  file:    Option<PathBuf>,
}

impl Worker {
  pub fn new(command: Command, iter: i64, sys: bool, file: Option<PathBuf>) -> Self {
    Self {
      command,
      iter,
      sys,
      file,
    }
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
      .unwrap()
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
        .context(error::StartupTime)?;

      child.wait().unwrap();

      let plugins = self.parse()?;

      for (k, v) in plugins.iter() {
        ret.entry(k.to_owned()).or_insert_with(Vec::new).push(*v);
      }

      self.clean()?;
    }

    Ok(
      ret
        .iter()
        .map(|(k, v)| Plugin::new(k.to_owned(), v.to_owned()))
        .collect::<Vec<Plugin>>(),
    )
  }

  /// Parse the contents of `vim.log`.
  ///
  /// 036.484  000.043  000.043: sourcing /path/to/plugin/file.vim
  ///                   ^^^^^^^                    ^^^^^^
  pub fn parse(&self) -> Result<HashMap<String, f64>> {
    let content = fs::read_to_string("vim.log").context(error::ReadLog)?;
    // in case the log contains windows-style path separators, they get replaced
    // with unix-style path separators.
    // that saves us from a more compicated regex pattern later on.
    let content = content.replace("\\","/");

    if let Some(plugin_directory) = self.plugin_directory(&content)? {
      let re = RegexBuilder::new(&format!(
        r"^\d+.\d+\s+\d+.\d+\s+(\d+.\d+): sourcing {}/([^/]+)/",
        plugin_directory
      ))
      .multi_line(true)
      .build()
      .unwrap();

      let mut plugins = HashMap::new();
      for capture in re.captures_iter(&content) {
        if let (Some(time), Some(plugin)) = (capture.get(1), capture.get(2)) {
          *plugins.entry(plugin.as_str().to_owned()).or_insert(0.0) +=
            time.as_str().parse::<f64>().unwrap();
        }
      }

      if self.sys {
        for dir in &vec!["/usr", "/usr/local"] {
          let re = RegexBuilder::new(&format!(
            r"^\d+.\d+\s+\d+.\d+\s+(\d+.\d+): sourcing {}/.+/([^/]+.vim)\n",
            dir
          ))
          .multi_line(true)
          .build()
          .unwrap();

          for capture in re.captures_iter(&content) {
            if let (Some(time), Some(plugin)) = (capture.get(1), capture.get(2)) {
              *plugins.entry(plugin.as_str().to_owned()).or_insert(0.0) +=
                time.as_str().parse::<f64>().unwrap();
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
  pub fn plugin_directory(&self, content: &str) -> Result<Option<String>> {
    let re = RegexBuilder::new(
      r"^\d+.\d+\s+\d+.\d+\s+\d+.\d+: sourcing (.+?)/(?:[^/]+/)(?:autoload|ftdetect|plugin|syntax)/[^/]+",
    ).multi_line(true).build().unwrap();

    let mut counts = HashMap::new();
    for capture in re.captures_iter(&content) {
      if let Some(directory) = capture.get(1) {
        if !directory.as_str().starts_with("/usr") {
          *counts.entry(directory.as_str()).or_insert(0) += 1;
        }
      }
    }

    Ok(
      counts
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k.to_string()),
    )
  }

  /// Clean up the created `vim.log` file.
  fn clean(&self) -> Result<()> {
    fs::remove_file("vim.log").context(error::RemoveLog)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn plugin_directory() {
    let content = r#"
      042.203  011.313  010.704: sourcing /usr/local/Cellar/neovim/0.5.0/share/nvim/runtime/filetype.vim
      065.646  001.389  000.393: sourcing /usr/local/Cellar/neovim/0.5.0/share/nvim/runtime/syntax/syntax.vim
      038.356  000.029  000.029: sourcing /Users/.vim/plugged/vim-prettier/ftdetect/graphql.vim
      039.955  000.028  000.028: sourcing /Users/.vim/plugged/vim-polyglot/ftdetect/polyglot.vim
      040.327  000.058  000.058: sourcing /Users/.vim/plugged/vim-markdown/ftdetect/markdown.vim
      040.530  000.048  000.048: sourcing /Users/.vim/plugged/rust.vim/ftdetect/rust.vim
    "#;

    let worker = Worker::new(Command::Vim, 1, false, None);
    assert_eq!(
      worker.plugin_directory(&dedent(content)).unwrap().unwrap(),
      "/Users/.vim/plugged"
    );
  }

  #[test]
  fn plugin_directory_empty_content() {
    let worker = Worker::new(Command::Vim, 1, false, None);
    assert!(worker.plugin_directory("").unwrap().is_none());
  }

  #[test]
  fn parse() {
    let content = r#"
      038.356  000.029  000.029: sourcing /Users/.vim/plugged/vim-prettier/ftdetect/graphql.vim
      039.955  000.028  000.028: sourcing /Users/.vim/plugged/vim-polyglot/ftdetect/polyglot.vim
      040.327  000.058  000.058: sourcing /Users/.vim/plugged/vim-markdown/ftdetect/markdown.vim
      040.530  000.048  000.048: sourcing /Users/.vim/plugged/rust.vim/ftdetect/rust.vim
    "#;

    let cases = vec![
      ("vim-prettier", 0.029),
      ("vim-polyglot", 0.028),
      ("vim-markdown", 0.058),
      ("rust.vim", 0.048),
    ];

    let mut file = fs::File::create("vim.log").unwrap();
    file.write_all(dedent(content).as_bytes()).unwrap();

    let worker = Worker::new(Command::Vim, 1, false, None);
    let data = worker.parse().unwrap();

    println!("{:?}", data);

    for (key, value) in cases {
      assert!(approx_eq!(f64, data[key], value, ulps = 2));
    }

    fs::remove_file("vim.log").unwrap();
  }
}
