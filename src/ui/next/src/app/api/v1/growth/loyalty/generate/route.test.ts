import { expect, test } from "vitest";
import { POST } from "./route";

test("fails closed when no real loyalty-generation service exists", async () => {
  const response = await POST(
    new Request("http://localhost/api/v1/growth/loyalty/generate", {
      method: "POST",
      body: JSON.stringify({ programName: "VIP Club" }),
    }),
  );

  expect(response.status).toBe(501);
  await expect(response.json()).resolves.toEqual({
    error: "loyalty generation is not implemented",
  });
});
