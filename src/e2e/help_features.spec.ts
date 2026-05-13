import { test, expect } from "@playwright/test";

test.describe("Help Center Features", () => {
    test("should fetch and render help center articles", async ({ request }) => {
        const response = await request.get("http://localhost:8080/api/help/articles");
        expect(response.status()).toBe(200);
        const data = await response.json();
        expect(data.length).toBeGreaterThan(0);
        expect(data[0].title).toBeDefined();
    });

    test("should fetch tooltips", async ({ request }) => {
        const response = await request.get("http://localhost:8080/api/help/tooltips");
        expect(response.status()).toBe(200);
        const data = await response.json();
        expect(data.length).toBeGreaterThan(0);
        expect(data[0].element_selector).toBeDefined();
    });

    test("should fetch walkthroughs", async ({ request }) => {
        const response = await request.get("http://localhost:8080/api/help/walkthroughs");
        expect(response.status()).toBe(200);
        const data = await response.json();
        expect(data.length).toBeGreaterThan(0);
        expect(data[0].target).toBeDefined();
    });

    test("should interact with AI chat", async ({ request }) => {
        const response = await request.post("http://localhost:8080/api/help/chat", {
            data: { message: "store" }
        });
        expect(response.status()).toBe(200);
        const data = await response.json();
        expect(data.reply).toContain("set up your store");
    });
});
