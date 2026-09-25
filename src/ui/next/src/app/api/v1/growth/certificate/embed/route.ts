import { NextRequest, NextResponse } from "next/server";

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url);
  const title = searchParams.get("title") || "Certificate of Achievement";
  const recipient = searchParams.get("recipient") || "John E2E Doe";
  const course =
    searchParams.get("course") ||
    searchParams.get("course_name") ||
    "E2E Mastery Course";
  const tenant = searchParams.get("tenant") || "default";

  const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>${title}</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      background: #f5f5f7;
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
      margin: 0;
      padding: 20px;
      box-sizing: border-box;
    }
    .cert-card {
      background: #fff;
      border: 2px solid #e5e5ea;
      border-radius: 16px;
      padding: 40px;
      max-width: 600px;
      width: 100%;
      text-align: center;
      box-shadow: 0 4px 20px rgba(0,0,0,0.06);
    }
    h1 {
      font-size: 28px;
      color: #1d1d1f;
      margin-bottom: 20px;
    }
    .recipient {
      font-size: 24px;
      font-weight: 700;
      color: #0066ff;
      margin: 20px 0;
    }
    h2 {
      font-size: 20px;
      color: #48484a;
      font-weight: 500;
      margin-bottom: 30px;
    }
    footer {
      border-top: 1px solid #f0f0f2;
      padding-top: 15px;
      font-size: 13px;
    }
    footer a {
      color: #86868b;
      text-decoration: none;
      font-weight: 600;
    }
    footer a:hover {
      color: #0066ff;
    }
  </style>
</head>
<body>
  <div class="cert-card">
    <h1>${title}</h1>
    <p>This certifies that</p>
    <div class="recipient">${recipient}</div>
    <p>has successfully completed</p>
    <h2>${course}</h2>
    <footer>
      <a href="/onboarding?ref=${encodeURIComponent(tenant)}">⚡ OmniSolo</a>
    </footer>
  </div>
</body>
</html>`;

  return new NextResponse(html, {
    headers: {
      "Content-Type": "text/html; charset=utf-8",
    },
  });
}
