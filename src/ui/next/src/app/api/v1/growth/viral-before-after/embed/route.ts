import { NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
    if (!unsafe) return '';
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
    const title = searchParams.get('title') || 'Our Work';
    const beforeUrl = searchParams.get('before') || 'https://images.unsplash.com/photo-1584622650111-993a426fbf0a?auto=format&fit=crop&q=80&w=800';
    const afterUrl = searchParams.get('after') || 'https://images.unsplash.com/photo-1527515637462-cff94eecc1ac?auto=format&fit=crop&q=80&w=800';
    const rawBranding = searchParams.get('branding') !== 'false';

    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${escapeHtml(title)}</title>
    <style>
        body {
            margin: 0;
            padding: 20px;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            display: flex;
            flex-direction: column;
            min-height: 100vh;
            background: transparent;
            box-sizing: border-box;
        }
        .widget-container {
            max-width: 600px;
            width: 100%;
            margin: auto;
            background: #ffffff;
            border-radius: 16px;
            overflow: hidden;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
            border: 1px solid #f3f4f6;
        }
        .header {
            padding: 16px 20px;
            background: #f9fafb;
            border-bottom: 1px solid #f3f4f6;
            text-align: center;
        }
        .title {
            font-size: 18px;
            font-weight: 600;
            color: #111827;
            margin: 0;
        }
        .slider-container {
            position: relative;
            width: 100%;
            height: 300px;
            overflow: hidden;
        }
        .img {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            object-fit: cover;
            pointer-events: none;
        }
        .img-after {
            z-index: 1;
        }
        .img-before {
            z-index: 2;
            clip-path: polygon(0 0, 50% 0, 50% 100%, 0 100%);
        }
        .slider-input {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            margin: 0;
            opacity: 0;
            cursor: ew-resize;
            z-index: 4;
        }
        .slider-line {
            position: absolute;
            top: 0;
            left: 50%;
            width: 4px;
            height: 100%;
            background: white;
            z-index: 3;
            transform: translateX(-50%);
            box-shadow: 0 0 10px rgba(0,0,0,0.5);
            pointer-events: none;
        }
        .slider-button {
            position: absolute;
            top: 50%;
            left: 50%;
            width: 32px;
            height: 32px;
            background: white;
            border-radius: 50%;
            z-index: 3;
            transform: translate(-50%, -50%);
            box-shadow: 0 2px 6px rgba(0,0,0,0.3);
            display: flex;
            align-items: center;
            justify-content: center;
            pointer-events: none;
        }
        .slider-button::before, .slider-button::after {
            content: '';
            display: inline-block;
            width: 0;
            height: 0;
            border-style: solid;
        }
        .slider-button::before {
            border-width: 5px 6px 5px 0;
            border-color: transparent #374151 transparent transparent;
            margin-right: 4px;
        }
        .slider-button::after {
            border-width: 5px 0 5px 6px;
            border-color: transparent transparent transparent #374151;
        }
        .label {
            position: absolute;
            bottom: 12px;
            padding: 4px 10px;
            background: rgba(0, 0, 0, 0.6);
            color: white;
            font-size: 12px;
            font-weight: 600;
            border-radius: 20px;
            backdrop-filter: blur(4px);
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .label-before {
            left: 12px;
            z-index: 3;
        }
        .label-after {
            right: 12px;
            z-index: 1;
        }
        .footer {
            text-align: center;
            font-size: 12px;
            padding: 12px;
            background: #f9fafb;
            border-top: 1px solid #f3f4f6;
        }
        .footer a {
            color: #6b7280;
            text-decoration: none;
            font-weight: 600;
            transition: color 0.2s;
        }
        .footer a:hover {
            color: #374151;
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="widget-container">
        <div class="header">
            <h2 class="title">${escapeHtml(title)}</h2>
        </div>
        <div class="slider-container" id="container">
            <img src="${escapeHtml(afterUrl)}" alt="After" class="img img-after" />
            <img src="${escapeHtml(beforeUrl)}" alt="Before" class="img img-before" id="beforeImage" />

            <div class="label label-before" id="beforeLabel">Before</div>
            <div class="label label-after">After</div>

            <div class="slider-line" id="sliderLine">
                <div class="slider-button"></div>
            </div>

            <input type="range" min="0" max="100" value="50" class="slider-input" id="slider" aria-label="Percentage of before photo shown" />
        </div>
        ${rawBranding ? `
        <div class="footer">
            <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}" target="_blank">⚡ Powered by OHC</a>
        </div>
        ` : ""}
    </div>

    <script>
        document.addEventListener('DOMContentLoaded', function() {
            const slider = document.getElementById('slider');
            const beforeImage = document.getElementById('beforeImage');
            const sliderLine = document.getElementById('sliderLine');
            const beforeLabel = document.getElementById('beforeLabel');

            function updateSlider() {
                const val = slider.value;
                beforeImage.style.clipPath = \`polygon(0 0, \${val}% 0, \${val}% 100%, 0 100%)\`;
                sliderLine.style.left = \`\${val}%\`;

                // Hide 'Before' label if it gets covered by the slider
                if (val < 20) {
                    beforeLabel.style.opacity = '0';
                } else {
                    beforeLabel.style.opacity = '1';
                }
            }

            slider.addEventListener('input', updateSlider);
            // Initial call
            updateSlider();
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
