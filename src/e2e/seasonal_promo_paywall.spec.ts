import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - seasonal_promo_paywall', () => {
  currentAppSmoke('seasonal_promo_paywall');
});
