import { chromium, type FullConfig } from '@playwright/test';
import { Pool } from 'pg';
import * as fs from 'fs';
import * as path from 'path';

export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for e2e global setup.');
  }

  const databaseURL = process.env.DATABASE_URL;
  if (!databaseURL) {
    throw new Error('DATABASE_URL is required to seed the test database. Ensure it is set in the environment.');
  }

  console.log('[globalSetup] Seeding database using URL:', databaseURL);
  const pool = new Pool({ connectionString: databaseURL });
  try {
    const seedSqlPath = path.join(__dirname, 'e2e-seed.sql');
    const seedSql = fs.readFileSync(seedSqlPath, 'utf8');
    await pool.query(seedSql);
    console.log('[globalSetup] Database seeded successfully.');
  } catch (error) {
    console.error('[globalSetup] Error seeding database:', error);
    throw error;
  } finally {
    await pool.end();
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
