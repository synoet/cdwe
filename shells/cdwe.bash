function cdwe() {
  local old_dir="$PWD"

  builtin cd "$@" || return

  local new_dir="$PWD"

  local result
  result="$(/Users/synoet/dev/cdwe/target/release/cdwe --old_dir="$old_dir" --new_dir="$new_dir")"
  eval "${result}"
}

