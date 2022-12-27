# How to use

## 0. Prerequisites

[Run chrome driver](Install.md)

## 1. Set permissions

1. Prepare XML file for import

   Prepare `import.xml` file. You can override filename with `--xml-file` option.

2. Prepare app config:

    ```shell
    cp spt.yml-dist spt.yml
    ```

   Edit and put valid credentials, tokens, etc.

3. Run:

    ```shell
    ./spt set
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
  - `--category`
  - `--client`
  - `--login-starts-with`
  - `--name-starts-with`

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
