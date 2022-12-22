# Installation

1. Install Chrome Web Driver

   **RedHat/CentOS:**

    ```shell
    yum -y install chromedriver
    ```

    **ArchLinux:**

    ```shell
    paru -S chromedriver
    ```

    For other OS check [official download page](https://chromedriver.chromium.org/downloads).

2. Install chrome

    ArchLinux:

    ```shell
    paru -S google-chrome
    ```
 
3. Add permission:

   ```shell
   chmod +x spt
   ```
  
4. Run chrome-driver

    ```shell
    $ chromedriver
   
    Starting ChromeDriver 108.0.5359.71 (1e0e3868ee06e91ad636a874420e3ca3ae3756ac-refs/branch-heads/5359@{#1016}) on port 9515
    Only local connections are allowed.
    Please see https://chromedriver.chromium.org/security-considerations for suggestions on keeping ChromeDriver safe.
    ChromeDriver was started successfully.
    ```
