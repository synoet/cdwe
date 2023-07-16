function cdwe
  set old_dir $PWD

  cd $argv; or return

  set new_dir $PWD

  set result (/Users/synoet/dev/cdwe/target/release/cdwe --old_dir="$old_dir" --new_dir="$new_dir")
  eval $result
end

