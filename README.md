# Permissions Tool for sysPass

Manage permissions for [SysPass](https://github.com/nuxsmin/sysPass) accounts.

## Features

1. Set permissions for accounts (Work in progress)

## Why?

1. sysPass allows to import data from csv / xml files or [API]((https://syspass-doc.readthedocs.io/en/3.1/application/api.html)), but it doesn't support permissions.

2. At the moment (14.12.2022) I didn't find suitable solution.

## Getting started

1. Prepare xml file for import

2. Create API Authorization token

   Go to `Configuration -> Users and Accesses -> API Authorizations`

   Create authorization for user with global view permissions -> Select action `Search for accounts` ->
   Create password -> Save

   View and remember API token

3. Prepare app config:

    ```shell
    cp sip.yml-dist sip.yml
    ```
   
    Use token password and api token from step 2.

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

3. Update permissions for accounts

    Uses chrome webdriver in headless mode.
