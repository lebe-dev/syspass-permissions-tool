# How it works

1. Read xml-file and extract properties:

    - Category Name
    - Client Name
    - Login

2. Search accounts via UI with values from xml

   Filter by properties: login, category and client

3. Update permissions for accounts

   Uses chrome webdriver.

## Progress cache for command

Commands `set` and `get-empty` creates file cache inside working directory.
