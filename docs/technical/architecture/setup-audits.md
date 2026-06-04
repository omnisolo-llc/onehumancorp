# OHC Setup Audit Documentation

To execute automated "Day One" setup flow audits:

1. Create an `.env` file with values for `OHC_PORT`, `LOG_LEVEL`, and `OHC_SOURCE_MODE`.
2. Run `deploy/scripts/ohc-verify-setup.sh` to audit the configuration and emit telemetry logs in Markdown and YAML formats.

```bash
cat << 'ENV' > .env
OHC_PORT=18789
LOG_LEVEL=info
OHC_SOURCE_MODE=standalone
ENV

deploy/scripts/ohc-verify-setup.sh
```

Audit reports are securely stored in `.ohc/runtime/status/`.
