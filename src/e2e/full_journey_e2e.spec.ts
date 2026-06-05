import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - full_journey_e2e', () => {
  currentAppSmoke('full_journey_e2e');
});
