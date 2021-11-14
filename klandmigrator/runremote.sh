#!/bin/sh

cargo build --release
# cargo build --target x86_64-unknown-linux-musl --release
rsync -v --progress -e 'ssh -p 241' Settings.toml target/release/klandmigrate csanchez@oboy.smilebasicsource.com:/home/csanchez/nobackup/sbsredux/klandmigrateexe
# rsync -v --progress -e 'ssh -p 241' Settings.toml target/x86_64-unknown-linux-musl/release/klandmigrate csanchez@oboy.smilebasicsource.com:/home/csanchez/nobackup/sbsredux/klandmigrateexe
ssh csanchez@oboy.smilebasicsource.com -p 241 'cd nobackup/sbsredux/klandmigrateexe && LD_LIBRARY_PATH=/opt/glibc-2.29/lib ./klandmigrate'
