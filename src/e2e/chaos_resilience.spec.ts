import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - chaos_resilience', () => {
  currentAppSmoke('chaos_resilience');
});
