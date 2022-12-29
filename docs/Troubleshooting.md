# Troubleshooting

SPT strongly depends on webdriver and sometimes web-driver crashes with errors:

> webDriver command error: webdriver returned error: unknown error: session deleted because of page crash

> webDriver command error: webdriver returned error: element not interactable

**Solutions:**

1. Restart web-driver process.

2. Use `--resume` flag for commands to continue process from checkpoint.
