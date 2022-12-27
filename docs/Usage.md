# How to use

## 0. Prerequisites

[Run chrome driver](Install.md)

## 1. Set permissions

1. [Create backup of your sysPass data](https://syspass-doc.readthedocs.io/en/3.1/application/backup.html)

2. Prepare XML file for import

   Prepare `import.xml` file. You can override filename with `--xml-file` option.

3. Prepare app config:

    ```shell
    cp spt.yml-dist spt.yml
    ```

   Edit and put valid credentials, tokens, etc.

4. Run:

    ```shell
    ./spt set --xml-file import.xml
    ```

## 2. Get empty permissions

Looking for accounts with empty permissions and print results in JSON format.

Run:

```shell
./spt get-empty [OPTIONS]
```

Options:

- `--resume` - try to continue process based on file cache.

- Account filters:
  - `--category <name>`
  - `--client <name>`
  - `--login-starts-with <name>`
  - `--name-starts-with <name>`

Example output:

```json
[
   {
     "name": "i.petrov",
     "login": "Ivan Petrov",
     "client": "AppStore",
     "category": "Frogs Ltd"
   }
]
```
