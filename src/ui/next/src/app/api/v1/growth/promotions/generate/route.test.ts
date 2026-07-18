import { describe, expect, it } from "vitest";
import { POST } from "./route";

describe("promotion generation API", () => {
  it("fails closed when no real promotion service is available", async () => {
    const response = await POST(
      new Request("http://localhost/api/v1/growth/promotions/generate", {
        method: "POST",
        body: JSON.stringify({ tenant: "bakery" }),
      }),
    );

    expect(response.status).toBe(501);
    await expect(response.json()).resolves.toEqual({
      error: "promotion generation is not implemented",
    });
  });
});
