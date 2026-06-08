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
    const rawTenant = searchParams.get('tenant') || 'demo';
    const rawTheme = searchParams.get('theme') || 'light';
    const rawTitle = searchParams.get('title') || 'Work Request';
    const rawBranding = searchParams.get('branding') !== 'false';

    const tenant = escapeHtml(rawTenant);
    const encodedTenant = encodeURIComponent(rawTenant);
    const theme = escapeHtml(rawTheme);
    const title = escapeHtml(rawTitle);

    const isDark = theme === 'dark';

    // Core OHC design tokens
    const colors = {
        bg: isDark ? '#1a1a1a' : '#ffffff',
        text: isDark ? '#f5f5f5' : '#111827',
        border: isDark ? '#333333' : '#e5e7eb',
        inputBg: isDark ? '#2d2d2d' : '#f9fafb',
        buttonBg: isDark ? '#ffffff' : '#111827',
        buttonText: isDark ? '#000000' : '#ffffff',
        muted: isDark ? '#9ca3af' : '#6b7280',
    };

    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${title}</title>
    <style>
        * { box-sizing: border-box; }
        body {
            margin: 0;
            padding: 20px;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            background-color: ${colors.bg};
            color: ${colors.text};
            -webkit-font-smoothing: antialiased;
        }

        .container {
            max-width: 100%;
            height: 100%;
            display: flex;
            flex-direction: column;
        }

        h2 {
            margin: 0 0 16px 0;
            font-size: 1.25rem;
            font-weight: 600;
        }

        .form-group {
            margin-bottom: 16px;
        }

        label {
            display: block;
            margin-bottom: 6px;
            font-size: 0.875rem;
            font-weight: 500;
        }

        input, textarea {
            width: 100%;
            padding: 10px 12px;
            border: 1px solid ${colors.border};
            border-radius: 8px;
            background-color: ${colors.inputBg};
            color: ${colors.text};
            font-family: inherit;
            font-size: 0.875rem;
            transition: border-color 0.2s, box-shadow 0.2s;
        }

        input:focus, textarea:focus {
            outline: none;
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
        }

        textarea {
            resize: vertical;
            min-height: 80px;
        }

        button {
            width: 100%;
            padding: 12px;
            background-color: ${colors.buttonBg};
            color: ${colors.buttonText};
            border: none;
            border-radius: 8px;
            font-weight: 600;
            font-size: 0.875rem;
            cursor: pointer;
            transition: opacity 0.2s;
            margin-top: 8px;
        }

        button:hover {
            opacity: 0.9;
        }

        button:active {
            transform: scale(0.98);
        }

        /* Loading Spinner */
        .spinner {
            display: none;
            width: 16px;
            height: 16px;
            border: 2px solid rgba(255,255,255,0.3);
            border-radius: 50%;
            border-top-color: currentColor;
            animation: spin 1s ease-in-out infinite;
            margin: 0 auto;
        }

        @keyframes spin {
            to { transform: rotate(360deg); }
        }

        button.loading span { display: none; }
        button.loading .spinner { display: block; }

        /* Success State */
        .success-state {
            display: none;
            text-align: center;
            padding: 32px 0;
            animation: fadeIn 0.4s ease;
        }

        .success-icon {
            font-size: 48px;
            margin-bottom: 16px;
        }

        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }

        .footer {
            margin-top: auto;
            padding-top: 20px;
            text-align: center;
            font-size: 0.75rem;
            color: ${colors.muted};
        }

        .footer a {
            color: inherit;
            text-decoration: none;
            font-weight: 600;
        }

        .footer a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <div id="form-container">
            <h2>${title}</h2>
            <form id="intake-form">
                <div class="form-group">
                    <label for="name">Name</label>
                    <input type="text" id="name" name="name" required placeholder="Jane Doe">
                </div>

                <div class="form-group">
                    <label for="email">Email</label>
                    <input type="email" id="email" name="email" required placeholder="jane@example.com">
                </div>

                <div class="form-group">
                    <label for="request">What do you need help with?</label>
                    <textarea id="request" name="request" required placeholder="Please describe your project or request..."></textarea>
                </div>

                <button type="submit" id="submit-btn">
                    <span>Send Request</span>
                    <div class="spinner"></div>
                </button>
            </form>
        </div>

        <div id="success-container" class="success-state">
            <div class="success-icon">✨</div>
            <h2 style="margin-bottom: 8px;">Request Received</h2>
            <p style="color: ${colors.muted}; font-size: 0.875rem;">We'll get back to you shortly.</p>
            <button id="reset-btn" style="background-color: transparent; color: ${colors.text}; border: 1px solid ${colors.border}; margin-top: 24px;">
                Send another request
            </button>
        </div>

        ${rawBranding ? `
        <div class="footer">
            ⚡ Powered by OHC
            <!-- Hidden link for crawler attribution and referral loop -->
            <span style="display:none;">
               <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}" target="_blank" rel="noopener noreferrer">OHC</a>
            </span>
        </div>
        ` : ''}
    </div>

    <script>
        document.getElementById('intake-form').addEventListener('submit', async (e) => {
            e.preventDefault();

            const btn = document.getElementById('submit-btn');
            const formContainer = document.getElementById('form-container');
            const successContainer = document.getElementById('success-container');

            // Loading state
            btn.classList.add('loading');
            btn.disabled = true;

            // In a real implementation, this would POST to the OHC backend
            // await fetch('/api/v1/work-intake/submit', { ... })

            // Simulate network request
            setTimeout(() => {
                btn.classList.remove('loading');
                btn.disabled = false;

                // Show success
                formContainer.style.display = 'none';
                successContainer.style.display = 'block';
            }, 1200);
        });

        document.getElementById('reset-btn').addEventListener('click', () => {
            document.getElementById('intake-form').reset();
            document.getElementById('success-container').style.display = 'none';
            document.getElementById('form-container').style.display = 'block';
        });
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
