import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - playwright_kairos_mesh', () => {
  currentAppSmoke('playwright_kairos_mesh');
});
