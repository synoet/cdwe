# cdwe (change directory with env vars)
A simple configurable cd wrapper that sets env vars per directory, based on a config file. \
\
Define environment variables per directory in a `~/cdwe.toml` file, and cdwe will set and uset environment variables corresponding dirs and subdirs.

![usage](./assets/usage.gif)

## Installation
1. **Install binary**
```bash
cargo install cdwe
```

2. **Create a cdwe.toml file in your home directory**
```toml
[[directory]]
path = "/Users/synoet/dev/projectb"
vars = { "TEST_VAR"= "Hello World", }

[[directory]]
path = "/Users/synoet/dev/projecta"
vars = { "IS_DEBUG"= "true", "IS_CI" = "false"}
```

3. **Init your shell**
```bash
cdwe init zsh # zsh shells
cdwe init bash # bash shells
cdwe init fish # fish shells
```

4. **Reload your shell and start using!**
```bash
# check that env var gets set
cdwe /Users/synoet/dev/projecta
echo $IS_DEBUG

# check that env var gets unset
cdwe ..
echo $IS_DEBUG
```


### What if I use something other than builtin cd?

1. You can define a `cd_command` in the config section of the `~/cdwe.toml`.
```toml
[config]
cd_command = "z" # if you are using zoxide

#... rest of config
```

2. Run `cdwe-reload`
```bash
cdwe-reload # reloads your config

zsh #reload your shell, use bash or fish if you use those.

```

### Uninstalling
1. Run cdwe-remove to clean up all shell artifacts
```bash
cdwe-remove #removes the `source <output>` from your .zshrc/.bashrc/.fish

zsh #reload your shell, use bash or fish if you use those.
```

2. Uninstall binary
```bash
cargo uninstall cdwe
```

