import { GET } from './route';

describe("GET /api/v1/growth/viral-streak/embed", () => {
  it("should return the HTML content with default params", async () => {
    const request = new Request("https://app.example.test/api/v1/growth/viral-streak/embed");
    const response = await GET(request);

    expect(response.status).toBe(200);
    const html = await response.text();
    expect(html).toContain("Daily Login Streak");
    expect(html).toContain("Check in for 7 days to unlock your reward!");
    expect(html).toContain("Free Coffee");
    expect(html).toContain("Powered by OHC");
  });

  it("should return the HTML content with custom params", async () => {
    const request = new Request("https://app.example.test/api/v1/growth/viral-streak/embed?tenant=my-test-tenant&theme=dark&title=Awesome+Streak&goal=30&reward=Super+Prize&branding=false");
    const response = await GET(request);

    expect(response.status).toBe(200);
    const html = await response.text();
    expect(html).toContain("Awesome Streak");
    expect(html).toContain("Check in for 30 days to unlock your reward!");
    expect(html).toContain("Super Prize");
    expect(html).not.toContain("Powered by OHC");
  });

  it("escapes html to prevent xss", async () => {
    const request = new Request("https://app.example.test/api/v1/growth/viral-streak/embed?title=<script>alert('xss')</script>&reward=<img src=x onerror=alert('xss')>");
    const response = await GET(request);

    expect(response.status).toBe(200);
    const html = await response.text();
    expect(html).not.toContain("<script>alert('xss')</script>");
    expect(html).toContain("&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
    expect(html).not.toContain("<img src=x onerror=alert('xss')>");
    expect(html).toContain("&lt;img src=x onerror=alert(&#x27;xss&#x27;)&gt;");
  });
});