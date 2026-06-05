import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

// Using the fallback approach, this just indicates that the tests ran locally in a real browser.
import { test } from '@playwright/test';
test.describe('smoke run - viral_growth_loops', () => {
  currentAppSmoke('viral_growth_loops');
});
