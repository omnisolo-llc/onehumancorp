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
    const rawTitle = searchParams.get('title') || 'Spin to Win!';
    const rawOffer = searchParams.get('offer') || 'Spin the wheel for a chance to win a discount on your next order.';
    const rawSlices = searchParams.get('slices');
    const rawBranding = searchParams.get('branding') !== 'false';

    const tenant = escapeHtml(rawTenant);
    const encodedTenant = encodeURIComponent(rawTenant);
    const title = escapeHtml(rawTitle);
    const offer = escapeHtml(rawOffer);

    let slices = [
        { label: '10% Off', value: '10OFF', color: '#F87171' },
        { label: 'No Luck', value: 'NONE', color: '#9CA3AF' },
        { label: 'Free Shipping', value: 'FREESHIP', color: '#60A5FA' },
        { label: '20% Off', value: '20OFF', color: '#34D399' },
        { label: 'No Luck', value: 'NONE', color: '#9CA3AF' },
        { label: '$5 Off', value: '5OFF', color: '#FBBF24' },
    ];

    if (rawSlices) {
        try {
            const parsed = JSON.parse(rawSlices);
            if (Array.isArray(parsed) && parsed.length === 6) {
                slices = parsed;
            }
        } catch (e) {
            // fallback
        }
    }

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
            background-color: #ffffff;
            color: #111827;
            -webkit-font-smoothing: antialiased;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
        }

        .container {
            width: 100%;
            max-width: 400px;
            text-align: center;
            display: flex;
            flex-direction: column;
            align-items: center;
        }

        h2 {
            margin: 0 0 8px 0;
            font-size: 1.5rem;
            font-weight: 700;
        }

        p {
            margin: 0 0 24px 0;
            font-size: 0.875rem;
            color: #6b7280;
        }

        .wheel-container {
            position: relative;
            width: 250px;
            height: 250px;
            border-radius: 50%;
            border: 4px solid #111827;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
            overflow: hidden;
            margin-bottom: 24px;
            transition: transform 3s cubic-bezier(0.25, 0.1, 0.25, 1);
        }

        .wheel {
            width: 100%;
            height: 100%;
            border-radius: 50%;
            background: conic-gradient(
                ${slices[0].color} 0deg 60deg,
                ${slices[1].color} 60deg 120deg,
                ${slices[2].color} 120deg 180deg,
                ${slices[3].color} 180deg 240deg,
                ${slices[4].color} 240deg 300deg,
                ${slices[5].color} 300deg 360deg
            );
        }

        .pointer {
            position: absolute;
            top: -10px;
            left: 50%;
            transform: translateX(-50%);
            width: 0;
            height: 0;
            border-left: 10px solid transparent;
            border-right: 10px solid transparent;
            border-top: 20px solid #111827;
            z-index: 10;
        }

        .center-btn {
            position: absolute;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            width: 60px;
            height: 60px;
            background: white;
            border-radius: 50%;
            border: 2px solid #111827;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: bold;
            cursor: pointer;
            z-index: 5;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
        }

        .center-btn:hover {
            background: #f9fafb;
        }

        .form-container {
            width: 100%;
            display: flex;
            flex-direction: column;
            gap: 12px;
        }

        input {
            width: 100%;
            padding: 12px;
            border: 1px solid #e5e7eb;
            border-radius: 8px;
            font-size: 0.875rem;
        }

        button.submit {
            width: 100%;
            padding: 12px;
            background-color: #111827;
            color: white;
            border: none;
            border-radius: 8px;
            font-weight: 600;
            cursor: pointer;
            transition: opacity 0.2s;
        }

        button.submit:hover { opacity: 0.9; }

        .result-container {
            display: none;
            width: 100%;
            background: #f0fdf4;
            border: 1px solid #bbf7d0;
            padding: 16px;
            border-radius: 12px;
            margin-top: 16px;
        }

        .result-code {
            font-family: monospace;
            font-size: 1.25rem;
            font-weight: bold;
            letter-spacing: 2px;
            color: #166534;
            padding: 8px;
            background: white;
            border-radius: 4px;
            margin: 8px 0;
            display: inline-block;
        }

        .footer {
            margin-top: 20px;
            text-align: center;
            font-size: 0.75rem;
            color: #6b7280;
        }

        .footer a {
            color: inherit;
            text-decoration: none;
            font-weight: 600;
        }
    </style>
</head>
<body>
    <div class="container">
        <h2>${title}</h2>
        <p>${offer}</p>

        <div style="position: relative;">
            <div class="pointer"></div>
            <div class="wheel-container" id="wheel-container">
                <div class="wheel"></div>
            </div>
            <div class="center-btn" id="spin-btn">SPIN</div>
        </div>

        <div class="form-container" id="form-container">
            <input type="email" id="email" placeholder="Enter email to unlock" required>
            <button class="submit" id="unlock-btn">Unlock Spin</button>
        </div>

        <div class="result-container" id="result-container">
            <h3 style="margin: 0 0 8px 0; color: #166534;">You won!</h3>
            <div id="result-text" style="color: #15803d; font-weight: 600;"></div>
            <div class="result-code" id="result-code"></div>
            <div style="font-size: 0.75rem; color: #166534;">Use this code at checkout.</div>
        </div>

        ${rawBranding ? `
        <div class="footer">
            ⚡ Powered by OHC
            <span style="display:none;">
               <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}" target="_blank" rel="noopener noreferrer">OHC</a>
            </span>
        </div>
        ` : ''}
    </div>

    <script>
        const slices = ${JSON.stringify(slices)};
        let canSpin = false;
        let isSpinning = false;

        document.getElementById('unlock-btn').addEventListener('click', () => {
            const email = document.getElementById('email').value;
            if (email && email.includes('@')) {
                canSpin = true;
                document.getElementById('form-container').style.display = 'none';
                document.getElementById('spin-btn').style.background = '#fcd34d';
                document.getElementById('spin-btn').style.border = '2px solid #047857';
                document.getElementById('spin-btn').style.color = '#047857';
            } else {
                alert('Please enter a valid email.');
            }
        });

        document.getElementById('spin-btn').addEventListener('click', () => {
            if (!canSpin) {
                alert('Please enter your email to unlock the spin!');
                return;
            }
            if (isSpinning) return;

            isSpinning = true;

            // Randomly select a winning slice (index 0 to 5)
            // Biased to avoid "No Luck" if we want to ensure conversion, but let's just do random for demo
            let winIndex = Math.floor(Math.random() * 6);
            if (slices[winIndex].value === 'NONE') {
                // bump to a winner 80% of time
                if (Math.random() > 0.2) {
                   winIndex = (winIndex + 1) % 6;
                   if (slices[winIndex].value === 'NONE') winIndex = (winIndex + 1) % 6;
                }
            }

            const sliceAngle = 360 / 6;
            // Target angle points to the center of the winning slice
            // Wheel starts with slice 0 at top right (0-60deg), so middle is 30deg
            // We want the middle of winIndex to end up at top (which is -90deg or 270deg relative to start)

            const spins = 5; // number of full rotations
            const targetAngle = spins * 360 - (winIndex * sliceAngle) - (sliceAngle / 2);

            const wheel = document.getElementById('wheel-container');
            wheel.style.transform = \`rotate(\${targetAngle}deg)\`;

            setTimeout(() => {
                const winner = slices[winIndex];
                if (winner.value === 'NONE') {
                    document.getElementById('result-container').style.display = 'block';
                    document.getElementById('result-container').style.background = '#f3f4f6';
                    document.getElementById('result-container').style.borderColor = '#d1d5db';
                    document.getElementById('result-container').querySelector('h3').innerText = 'Oh no!';
                    document.getElementById('result-container').querySelector('h3').style.color = '#374151';
                    document.getElementById('result-text').innerText = 'Better luck next time.';
                    document.getElementById('result-text').style.color = '#4b5563';
                    document.getElementById('result-code').style.display = 'none';
                    document.getElementById('result-container').querySelector('div:last-child').style.display = 'none';
                } else {
                    document.getElementById('result-container').style.display = 'block';
                    document.getElementById('result-text').innerText = winner.label;
                    document.getElementById('result-code').innerText = winner.value;
                }
            }, 3100);
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
