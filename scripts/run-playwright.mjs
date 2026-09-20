import { runNativeE2e } from './native-e2e.mjs';

runNativeE2e().catch((error) => {
  console.error('[native-e2e]', error.message);
  process.exitCode = 1;
});
