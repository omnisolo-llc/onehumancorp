import { proxyBackendRequest } from "@/lib/auth/backendTransport";

export async function GET(request: Request) {
  const url = new URL(request.url);
  const tenant =
    url.searchParams.get("tenant_id") ||
    url.searchParams.get("tenant") ||
    "test-merchant-xyz";
  const widgetType = url.searchParams.get("type") || "quote";
  const theme = url.searchParams.get("theme") || "light";

  try {
    const res = await proxyBackendRequest(
      request,
      `/api/v1/growth/embed/widget${url.search}`,
    );
    if (res.ok) {
      return res;
    }
  } catch {
    // Fall back to direct generation if backend proxy fails
  }

  const bgColor = theme === "dark" ? "#1d1d1f" : "#ffffff";
  const textColor = theme === "dark" ? "#f5f5f7" : "#1d1d1f";

  const actionText =
    widgetType === "quote"
      ? "Request a quote"
      : widgetType === "booking"
        ? "Book an appointment"
        : "Connect with us";

  const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>OmniSolo Embed Widget</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      background-color: ${bgColor};
      color: ${textColor};
      margin: 0;
      padding: 16px;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      box-sizing: border-box;
    }
    .widget-container {
      width: 100%;
      max-width: 320px;
      padding: 20px;
      border-radius: 12px;
      box-shadow: 0 4px 6px rgba(0,0,0,0.1);
      text-align: center;
      border: 1px solid rgba(128,128,128,0.2);
    }
    .widget-title {
      font-size: 1.1rem;
      font-weight: 600;
      margin-bottom: 8px;
    }
    .widget-desc {
      font-size: 0.85rem;
      color: #888888;
      margin-bottom: 16px;
    }
    .btn {
      background-color: #0066FF;
      color: white;
      border: none;
      padding: 10px 16px;
      border-radius: 6px;
      font-weight: 500;
      cursor: pointer;
      width: 100%;
    }
    .watermark {
      margin-top: 16px;
      font-size: 0.75rem;
      color: #888888;
      text-decoration: none;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 4px;
    }
  </style>
</head>
<body>
  <div class="widget-container">
    <div class="widget-title">${actionText}</div>
    <div class="widget-desc">Workspace: ${tenant}</div>
    <button class="btn">${actionText}</button>
    <a href="https://omnisolo.co/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" class="watermark">
      <span>⚡ OmniSolo</span>
    </a>
  </div>
</body>
</html>`;

  return new Response(html, {
    status: 200,
    headers: { "content-type": "text/html; charset=utf-8" },
  });
}
