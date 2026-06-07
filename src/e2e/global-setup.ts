import { type FullConfig } from '@playwright/test';
import fs from 'node:fs';
import path from 'node:path';
import { Client } from 'pg';

export default async function globalSetup(config: FullConfig) {
  const baseURL = config.projects[0]?.use?.baseURL as string | undefined;
  if (!baseURL) {
    throw new Error('Playwright baseURL is required for e2e global setup.');
  }

  const databaseUrl = process.env.DATABASE_URL;
  if (!databaseUrl) {
    throw new Error('DATABASE_URL is required in the environment to seed the test database.');
  }

  // wait for app to be ready
  let isReady = false;
  for (let attempt = 0; attempt < 120; attempt += 1) {
    try {
      const response = await fetch(new URL('/login', baseURL));
      if (response.ok) {
        isReady = true;
        break;
      }
    } catch {
      // App is still booting.
    }
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }

  if (!isReady) {
    throw new Error('App failed to become ready for e2e setup.');
  }

  // Seed PostgreSQL database
  if (databaseUrl.startsWith('postgres://')) {
    console.log(`[playwright] Seeding database at ${databaseUrl.replace(/:[^:@]+@/, ':***@')}...`);
    const client = new Client({
      connectionString: databaseUrl,
    });

    try {
      await client.connect();
      // Read and execute e2e-seed.sql
      const seedSql = fs.readFileSync(path.join(__dirname, 'e2e-seed.sql'), 'utf-8');
      await client.query(seedSql);
      console.log('[playwright] Database seeded successfully.');
    } catch (err) {
      console.error('[playwright] Failed to seed database:', err);
      throw err;
    } finally {
      await client.end();
    }
  } else {
    console.log(`[playwright] Skipping Postgres seed for database URL: ${databaseUrl}`);
  }
}
