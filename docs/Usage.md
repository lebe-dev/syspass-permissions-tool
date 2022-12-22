# How to use

## 0. Prerequisites

[Run chrome driver](docs/Install.md)

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

Run:

```shell
./spt get-empty
```

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
