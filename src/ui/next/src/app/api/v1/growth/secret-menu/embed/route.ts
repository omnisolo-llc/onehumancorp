import { NextRequest, NextResponse } from 'next/server';

// Helper to prevent XSS by escaping HTML entities
function escapeHtml(unsafe: string) {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

export async function GET(req: NextRequest) {
  const searchParams = req.nextUrl.searchParams;

  const itemName = escapeHtml(searchParams.get('item_name') || 'Secret Item');
  const itemDesc = escapeHtml(searchParams.get('item_desc') || 'Unlock this secret item.');
  const accessCode = escapeHtml(searchParams.get('access_code') || 'SECRET');
  const sharesReq = escapeHtml(searchParams.get('shares_req') || '3');

  // In a real application, this would track shares and validate access.
  // For the generator preview and E2E test, we render a static visual.

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Secret Menu Item</title>
      <style>
        body {
          font-family: system-ui, -apple-system, sans-serif;
          background-color: #1a1a1a;
          color: white;
          display: flex;
          align-items: center;
          justify-content: center;
          min-height: 100vh;
          margin: 0;
          padding: 20px;
          text-align: center;
        }
        .card {
          background: #2a2a2a;
          border-radius: 16px;
          padding: 32px;
          max-width: 400px;
          width: 100%;
          box-shadow: 0 10px 30px rgba(0,0,0,0.5);
          border: 1px solid #444;
        }
        h1 { margin-top: 0; color: #ff6b6b; font-size: 24px; }
        p { color: #ccc; line-height: 1.5; }
        .lock-icon { font-size: 48px; margin-bottom: 16px; }
        .share-btn {
          background: #ff6b6b;
          color: white;
          border: none;
          padding: 12px 24px;
          border-radius: 8px;
          font-size: 16px;
          font-weight: bold;
          cursor: pointer;
          width: 100%;
          margin-top: 24px;
          transition: background 0.2s;
        }
        .share-btn:hover { background: #ff5252; }
        .progress {
          margin-top: 16px;
          font-size: 14px;
          color: #aaa;
        }
      </style>
    </head>
    <body>
      <div class="card">
        <div class="lock-icon">🔒</div>
        <h1>${itemName}</h1>
        <p>${itemDesc}</p>

        <div class="progress">
          <strong>0 / ${sharesReq}</strong> shares completed to unlock
        </div>

        <button class="share-btn">Share to Unlock</button>
      </div>
    </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: { 'Content-Type': 'text/html' },
  });
}
