#!/bin/bash
export PLAYWRIGHT_BASE_URL="http://localhost:8080"
export APP_SCREENSHOT_OUTPUT_DIR="$(pwd)/docs/public/assets/screenshots/app"
bazelisk run //srcs/app:capture_screenshots
