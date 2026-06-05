import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - virtual_meeting_walkthrough', () => {
  currentAppSmoke('virtual_meeting_walkthrough');
});
