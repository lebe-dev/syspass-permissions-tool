# How to use

## 1. Set permissions

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
