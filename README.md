# Permissions Tool for Syspass

Set permissions for accounts to [syspass](https://github.com/nuxsmin/sysPass).

## Features

1. Set permissions for accounts (Work in progress)

## Why?

1. Syspass allows to import data from csv / xml files or [API]((https://syspass-doc.readthedocs.io/en/3.1/application/api.html)), but it doesn't support permissions.

2. At the moment (14.12.2022) I didn't find suitable solution.

## Getting started

1. Prepare xml file for import

2. Create API Authorization token

    Configuration -> Users and Accesses

3. Prepare app config:

    ```shell
    cp sip.yml-dist sip.yml
    ```

4. Run:

    ```shell
    chmod +x spt
    ./spt --ignore-errors=false import.xml
    ```

## How it works

1. Read xml-file and extract properties:

- Name
- Category Name
- Client Name
- Login

2. Search accounts via [API]((https://syspass-doc.readthedocs.io/en/3.1/application/api.html)) with values from xml

3. If account was found, set required permissions
