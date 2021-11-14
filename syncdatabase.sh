#!/bin/sh

DBNAME=kland.db

# Since the migrator only really works well ON the server, we have a separate
# binary built on there that creates the sqlite database.

# Don't forget that it's difficult to connect to smilebasicsource.com. Talk to
# the repository owner if you want access to that machine (but it's unlikely)
rsync --progress -e 'ssh -p 241' \
   csanchez@oboy.smilebasicsource.com:~/nobackup/sbsredux/krustland/klandmigrator/${DBNAME} .
rsync --progress ./${DBNAME} \
   csanchez@smilebasicsource.com:~/projects/krustland/${DBNAME}
