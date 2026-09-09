import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

describe("E2E database helper", () => {
  const originalDatabaseUrl = process.env.DATABASE_URL;

  beforeEach(() => {
    process.env.DATABASE_URL = "postgres://e2e:test@127.0.0.1:5432/e2e";
    vi.resetModules();
  });

  afterEach(() => {
    if (originalDatabaseUrl === undefined) {
      delete process.env.DATABASE_URL;
    } else {
      process.env.DATABASE_URL = originalDatabaseUrl;
    }
  });

  it("exports the query helper used by database-backed browser tests", async () => {
    const { db, e2eDbQuery } = await import("../../../../e2e/db_utils");

    expect(e2eDbQuery).toEqual(expect.any(Function));
    expect(db.query).toBe(e2eDbQuery);
  });
});
