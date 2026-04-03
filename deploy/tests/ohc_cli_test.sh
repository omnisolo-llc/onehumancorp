#!/bin/bash
set -e

echo "Testing ohc_hybrid_cli.sh in non-interactive mode..."
output=$(bash ohc_hybrid_cli.sh --non-interactive)

if [[ "$output" == *"Verification completed."* ]]; then
  echo "Non-interactive check passed."
else
  echo "Non-interactive check failed."
  false
fi

if grep -q "run_doctor()" ohc_hybrid_cli.sh; then
  echo "run_doctor exists."
else
  echo "run_doctor missing."
  false
fi

if grep -q "GEMINI_API_KEY" ohc_hybrid_cli.sh; then
  echo "API Keys exist in wizard."
else
  echo "API keys missing."
  false
fi

echo "CLI test complete."
