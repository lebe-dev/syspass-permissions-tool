# Permissions Tool for sysPass

Manage permissions for [sysPass](https://github.com/nuxsmin/sysPass) accounts.

## Features

1. Set permissions for accounts
2. Get accounts without permissions (Work in progress)

## Why?

1. sysPass allows to import data from csv / xml files or [API]((https://syspass-doc.readthedocs.io/en/3.1/application/api.html)), 
but it doesn't support permissions.

2. At the moment (14.12.2022) I didn't find suitable solution.`

## Usage

1. [Install and run chrome driver](docs/Install.md)

2. Prepare XML file for import

   Prepare `import.xml` file. You can override filename with `--xml-file` option.

3. Prepare app config:

    ```shell
    cp spt.yml-dist spt.yml
    ```
   
    Edit and put valid credentials, tokens, etc.

4. Run:

    ```shell
    chmod +x spt
    ./spt set
    ```

## How it works

See [Architecture.md](docs/Architecture.md).

## For developers

See [Dev.md](docs/Dev.md).
