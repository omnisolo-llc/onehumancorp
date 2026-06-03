import { test, expect } from "@playwright/test";

test('Playwright environment has expected DATABASE_URL', async () => {
  const dbUrl = process.env.DATABASE_URL;
  expect(dbUrl).toBeDefined();

  if (process.env.CI) {
      expect(dbUrl).not.toContain('localhost:5432');
  }
});
