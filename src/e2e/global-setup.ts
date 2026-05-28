import { chromium, type FullConfig } from '@playwright/test';

import { Client } from "pg";

export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for e2e global setup.');
  }


  const dbUrl = process.env.DATABASE_URL;
  if (!dbUrl) {
    console.warn("DATABASE_URL is missing, skipping seeding");
  } else {
    try {
      const client = new Client({ connectionString: dbUrl });
      await client.connect();
      const seedSql = require('fs').readFileSync('src/e2e/e2e-seed.sql', 'utf8');
      await client.query(seedSql);
      await client.end();
      console.log("Database seeded successfully via " + dbUrl);
    } catch (e) {
      console.error("Failed to seed database:", e);
    }
  }


  // wait for app to be ready
  for (let attempt = 0; attempt < 60; attempt += 1) {
    try {
      const response = await fetch(new URL('/', baseURL));
      if (response.ok) return;
    } catch {
      // App is still booting.
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}
