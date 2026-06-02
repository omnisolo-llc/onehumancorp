# User Guide Screenshots

## E2E Testing (Playwright)

To ensure the Voice Agent E2E suite runs reliably in CI or locally, do not mock network requests. Follow these steps to start the Next.js server and run the Playwright tests:

```bash
# 1. Start the Next.js development server
cd src/ui/next
npm run dev &
NEXT_PID=$!

# 2. Wait for the server to be ready on port 3000
while ! nc -z localhost 3000; do
  sleep 1
done

# 3. Run the Playwright tests
cd ../../..
npx playwright test src/e2e/voice-agent.spec.ts

# 4. Clean up
kill $NEXT_PID
```

## E2E Testing (Playwright)

To ensure the Voice Agent E2E suite runs reliably in CI or locally, do not mock network requests. Follow these steps to start the Next.js server and run the Playwright tests:

```bash
# 1. Start the Next.js development server
cd src/ui/next
npm run dev &
NEXT_PID=$!

# 2. Wait for the server to be ready on port 3000
while ! nc -z localhost 3000; do
  sleep 1
done

# 3. Run the Playwright tests
cd ../../..
npx playwright test src/e2e/voice-agent.spec.ts

# 4. Clean up
kill $NEXT_PID
```
