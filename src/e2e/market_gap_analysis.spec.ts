import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - market_gap_analysis', () => {
  currentAppSmoke('market_gap_analysis');
});
