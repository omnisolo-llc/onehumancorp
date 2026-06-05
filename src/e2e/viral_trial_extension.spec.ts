import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - viral_trial_extension', () => {
  currentAppSmoke('viral_trial_extension');
});
