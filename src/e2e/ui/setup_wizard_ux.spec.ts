import { test, expect } from '@playwright/test';
import * as fs from 'fs';
import * as path from 'path';

test.describe('OHC Premium Setup Wizard UX CSS Verification', () => {

  test('setup.html CSS aligns with border-radius design guide', async () => {
    // We statically analyze the setup.html file to ensure the radius values
    // are configured correctly to 8px for the form controls.
    const setupFilePath = path.join(__dirname, '../../../src/ui/tauri/src/ui/setup.html');
    const content = fs.readFileSync(setupFilePath, 'utf8');

    // We check that button, input, select, textarea block uses 8px.
    expect(content).toContain('border-radius: 8px !important;');
    // Ensure we don't have the old bad config
    expect(content).not.toContain('border-radius: 16px !important;');
  });

});
