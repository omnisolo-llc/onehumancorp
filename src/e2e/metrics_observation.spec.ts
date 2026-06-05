import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - metrics_observation', () => {
  currentAppSmoke('metrics_observation');
});
