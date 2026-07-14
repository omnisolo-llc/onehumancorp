import { describe, expect, test } from "vitest";
import { subscriptionBackendPath } from "./subscriptionBackend";

describe("subscription backend paths", () => {
  test("confines subscription IDs", () => {
    expect(subscriptionBackendPath("sub-7")).toBe("/api/subscriptions/sub-7");
    expect(subscriptionBackendPath("sub-7", "/action")).toBe(
      "/api/subscriptions/sub-7/action",
    );
    expect(() => subscriptionBackendPath("../admin")).toThrow("invalid subscription ID");
  });

  test.each([".", ".."])('rejects exact dot-segment subscription ID "%s"', (id) => {
    expect(() => subscriptionBackendPath(id)).toThrow("invalid subscription ID");
  });
});
