# OmniSolo Setup Audit Documentation

To execute automated "Day One" setup flow audits:

1. Create an `.env` file with values for `OMNISOLO_PORT`, `LOG_LEVEL`, and `OMNISOLO_SOURCE_MODE`.
2. Run `deploy/scripts/omnisolo-verify-setup.sh` to audit the configuration and emit telemetry logs in Markdown and YAML formats.

```bash
cat << 'ENV' > .env
OMNISOLO_PORT=18789
LOG_LEVEL=info
OMNISOLO_SOURCE_MODE=standalone
ENV

deploy/scripts/omnisolo-verify-setup.sh
```

Audit reports are securely stored in `.omnisolo/runtime/status/`.
