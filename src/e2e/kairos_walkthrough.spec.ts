import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - kairos_walkthrough', () => {
  currentAppSmoke('kairos_walkthrough');
});
