import { type FullConfig } from '@playwright/test';

export default async function globalSetup(config: FullConfig) {
  // Simple dummy setup for our dummy test
  console.log("Dummy global setup completed");
}
