<div align="center">

# cdwe (cd with env vars)
A simple configurable cd wrapper that sets env vars per directory, based on a config file.

Define environment variables per directory in a `~/cdwe.toml` file, and cdwe will set and uset environment variables in corresponding directories and subdirectories. \

[Installation](#installation) •
[Configuration](#configuration) •
[Uninstalling](#uninstalling)
</div>

 


![usage](./assets/usage.gif)

## Installation

:warning: Behavior on bash/fish was not thoroughly tested. If you encounter any issues please create an issue.

1. **Install binary**
```bash
cargo install cdwe
```

2. **Init your shell**
```bash
cdwe init zsh # zsh shells
cdwe init bash # bash shells
cdwe init fish # fish shells
```

3. **Reload your shell and start using!**
```bash
# check that env var gets set
cdwe /Users/synoet/dev/projecta
echo $IS_DEBUG

# check that env var gets unset
cdwe ..
echo $IS_DEBUG
```


## Configuration 
#### Setting directory environment variables
```toml
[[directory]]
path = "/Users/synoet/dev/something" # the path to the directory
vars = {"IS_DEBUG" = "true", "IS_PROD" = "false"} # set environment variables
load_from = [".env", ".env.local"] # will source environment variables from these files

[[directory]]
path = "/Users/synoet/dev" # the path to the directory
load_from = [".env"] # will source environment variables from these files
```

#### What if I use something other than builtin cd?

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

## Uninstalling
1. Run cdwe-remove to clean up all shell artifacts
```bash
cdwe-remove #removes the `source <output>` from your .zshrc/.bashrc/.fish

zsh #reload your shell, use bash or fish if you use those.
```

2. Uninstall binary
```bash
cargo uninstall cdwe
```

