import { currentAppSmoke } from './current_app_smoke';

import { test } from '@playwright/test';
test.describe('smoke run - portfolio_generator', () => {
  currentAppSmoke('portfolio_generator');
});
