import { describe, expect, it } from "vitest";
import { POST } from "./route";

describe("social-proof generation API", () => {
  it("fails closed when no real social-proof service is available", async () => {
    const response = await POST(
      new Request("http://localhost/api/v1/growth/social-proof/generate", {
        method: "POST",
        body: JSON.stringify({ productName: "Celebration cake" }),
      }),
    );

    expect(response.status).toBe(501);
    await expect(response.json()).resolves.toEqual({
      error: "social-proof generation is not implemented",
    });
  });
});
