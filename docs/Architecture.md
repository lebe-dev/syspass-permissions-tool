# How it works

1. Read xml-file and extract properties:

    - Category Name
    - Client Name
    - Login

2. Search accounts via UI with values from xml

   Filter by properties: login, category and client

3. Update permissions for accounts

   Uses chrome webdriver.

## File cache

Command `get-empty` creates file cache (filename `accounts-get.cache`) inside working directory.
