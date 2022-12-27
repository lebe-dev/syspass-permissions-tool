# Troubleshooting

SPT strongly depends on webdriver and sometimes web-driver crashes with errors:

> webDriver command error: webdriver returned error: unknown error: session deleted because of page crash

> webDriver command error: webdriver returned error: element not interactable

**Solutions:**

1. Restart web-driver process

2. For `set` permissions command:
   1. Look for latest account properties in `spt.log`
   2. Cut from `import.xml` already processed accounts.
   3. Start `spt` tool again
