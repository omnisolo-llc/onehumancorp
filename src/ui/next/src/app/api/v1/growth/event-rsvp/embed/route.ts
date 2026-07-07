import { NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
    if (!unsafe) return unsafe;
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
}

export async function GET(request: Request) {
    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'demo';
    const title = searchParams.get('title') || 'Upcoming Event';
    const date = searchParams.get('date') || 'TBA';
    const location = searchParams.get('location') || 'TBA';
    const theme = searchParams.get('theme') || 'light';
    const rawBranding = searchParams.get('branding') !== 'false';

    const encodedTenant = encodeURIComponent(tenant);
    const isDark = theme === 'dark';

    const bg = isDark ? '#1D1D1F' : '#ffffff';
    const text = isDark ? '#ffffff' : '#111827';
    const textSecondary = isDark ? '#a1a1aa' : '#6b7280';
    const border = isDark ? '#333333' : '#e5e7eb';
    const inputBg = isDark ? '#333333' : '#f9fafb';
    const buttonBg = '#0066FF';
    const buttonHoverBg = '#005CE6';

    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${escapeHtml(title)} - RSVP</title>
    <style>
        body { margin: 0; padding: 16px; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background: transparent; box-sizing: border-box; }
        .widget-card {
            display: flex; flex-direction: column;
            background: ${bg}; color: ${text};
            border: 1px solid ${border}; border-radius: 16px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            max-width: 320px; width: 100%; box-sizing: border-box; overflow: hidden;
            transition: transform 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
        }
        .header {
            background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
            color: white; padding: 24px 20px; text-align: center;
        }
        .content-container {
            padding: 20px; display: flex; flex-direction: column; gap: 16px;
        }
        .title { font-size: 20px; font-weight: 700; margin: 0; line-height: 1.2; text-align: center; }
        .detail-row { display: flex; align-items: flex-start; gap: 8px; font-size: 14px; color: ${textSecondary}; }
        .detail-icon { font-size: 16px; line-height: 1; }
        .detail-text { margin: 0; line-height: 1.4; }

        .form-group { display: flex; flex-direction: column; gap: 6px; }
        .form-label { font-size: 12px; font-weight: 600; color: ${textSecondary}; }
        .form-input {
            padding: 10px 12px; border: 1px solid ${border}; border-radius: 8px;
            font-size: 14px; background: ${inputBg}; color: ${text};
            outline: none; transition: border-color 0.2s;
        }
        .form-input:focus { border-color: ${buttonBg}; }

        .rsvp-button {
            display: block; width: 100%; padding: 12px 0; margin-top: 4px;
            background: ${buttonBg}; color: #ffffff; text-align: center; text-decoration: none;
            font-size: 15px; font-weight: 600; border-radius: 8px; border: none; cursor: pointer;
            transition: background 0.2s ease-in-out;
        }
        .rsvp-button:hover { background: ${buttonHoverBg}; }

        .footer { text-align: center; font-size: 11px; margin-top: 12px; padding-top: 12px; border-top: 1px solid ${border}; }
        .footer a { color: ${textSecondary}; text-decoration: none; font-weight: 600; opacity: 0.8; transition: opacity 0.2s ease-in-out; }
        .footer a:hover { text-decoration: underline; opacity: 1; }

        .success-state { display: none; text-align: center; padding: 32px 20px; }
        .success-icon { font-size: 48px; margin-bottom: 16px; }
    </style>
</head>
<body>
    <div class="widget-card" id="rsvp-card">
        <div class="header">
            <h3 class="title">${escapeHtml(title)}</h3>
        </div>
        <div class="content-container">
            <div class="detail-row">
                <span class="detail-icon">📅</span>
                <p class="detail-text">${escapeHtml(date)}</p>
            </div>
            <div class="detail-row">
                <span class="detail-icon">📍</span>
                <p class="detail-text">${escapeHtml(location)}</p>
            </div>

            <div class="form-group">
                <label class="form-label">Full Name</label>
                <input type="text" class="form-input" placeholder="Jane Doe" id="rsvp-name">
            </div>
            <div class="form-group">
                <label class="form-label">Email Address</label>
                <input type="email" class="form-input" placeholder="jane@example.com" id="rsvp-email">
            </div>

            <button class="rsvp-button" onclick="submitRSVP()">RSVP Now</button>

            ${rawBranding ? `
            <div class="footer">
                <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_event_rsvp_widget" target="_blank">⚡ Powered by OHC</a>
            </div>
            ` : ''}
        </div>

        <div class="success-state" id="success-state">
            <div class="success-icon">🎉</div>
            <h3 style="margin-top:0;margin-bottom:8px;">You're on the list!</h3>
            <p style="color:${textSecondary};font-size:14px;margin-bottom:24px;">Check your email for details.</p>
            ${rawBranding ? `
                <div style="background:${inputBg};padding:16px;border-radius:12px;border:1px solid ${border};margin-bottom:8px;">
                    <p style="margin-top:0;font-size:13px;font-weight:600;">Want to host your own events?</p>
                    <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_event_rsvp_success" target="_blank" class="rsvp-button" style="background:#111827;padding:10px 0;font-size:13px;margin-top:12px;">Get Started with OHC</a>
                </div>
            ` : ''}
        </div>
    </div>

    <script>
        function submitRSVP() {
            var name = document.getElementById('rsvp-name').value;
            var email = document.getElementById('rsvp-email').value;
            var btn = document.querySelector('.rsvp-button');

            if (!name || !email) {
                alert('Please fill out all fields');
                return;
            }

            btn.textContent = 'Saving...';
            btn.style.opacity = '0.7';

            // Simulate network request
            setTimeout(function() {
                document.querySelector('.content-container').style.display = 'none';
                document.getElementById('success-state').style.display = 'block';
            }, 800);
        }
    </script>
</body>
</html>
    `;

    return new NextResponse(html, {
        headers: {
            'Content-Type': 'text/html; charset=utf-8',
            'Cache-Control': 'public, max-age=300, s-maxage=300'
        },
    });
}
