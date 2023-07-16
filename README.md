# cdwe (change directory with env vars)
A simple configurable cd wrapper that sets env vars per directory, based on a config file.

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
