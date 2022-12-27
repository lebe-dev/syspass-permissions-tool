# Permissions Tool for sysPass

Manage permissions for [sysPass](https://github.com/nuxsmin/sysPass) accounts.

## Why?

1. sysPass allows to import data from csv / xml files or [API]((https://syspass-doc.readthedocs.io/en/3.1/application/api.html)), 
but it doesn't support permissions.

2. At the moment (14.12.2022) I didn't find suitable solution.

3. sysPass project is abandoned by author

## Features

1. Set permissions for accounts
2. Get accounts without permissions

## Usage

See [Usage.md](docs/Usage.md).

## How it works

See [Architecture.md](docs/Architecture.md).

## Performance, resources and stability

SPT is ultra-fast and eats 13 KB of RAM, but strongly depends on chrome webdriver. You can tweak delays in config file.

### Get empty permissions and checkpoints

I did tests for `get-empty` command with Core i7 12650H and 1800+ accounts, webdriver ate 2-3 GB of RAM and crashed on 70% with message:

> error: webDriver command error: webdriver returned error: unknown error: session deleted because of page crash

SPT saves progress in file cache. Use `--resume` option for `get-empty` command if you want to continue interrupted process.

Also headless mode for webdriver is preferable.

## For developers

See [Dev.md](docs/Dev.md).

## RoadMap

1. Show progress in stdout
2. Performance tweaks
