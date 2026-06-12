// ohc-docs-features.js
// Consolidates Tooltips, Walkthroughs, and Help Chat for OHC

document.addEventListener('DOMContentLoaded', () => {

    // --- TOOLTIPS ---
    const tooltipStyle = document.createElement('style');
    tooltipStyle.textContent = `
        .ohc-tooltip {
            position: absolute;
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(20px) saturate(210%);
            -webkit-backdrop-filter: blur(20px) saturate(210%);
            border: 1px solid rgba(0, 102, 255, 0.2);
            color: #1d1d1f;
            padding: 8px 12px;
            border-radius: 8px;
            font-size: 13px;
            font-family: Outfit, sans-serif;
            pointer-events: none;
            z-index: 10000;
            opacity: 0;
            transition: opacity 0.2s, transform 0.2s;
            transform: translateY(5px);
            white-space: nowrap;
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
        }
        .ohc-tooltip.visible {
            opacity: 1;
            transform: translateY(0);
        }
        @media (prefers-color-scheme: dark) {
            .ohc-tooltip {
                background: rgba(22, 22, 26, 0.9);
                border-color: rgba(255, 255, 255, 0.1);
                color: #f5f5f7;
            }
        }
    `;
    document.head.appendChild(tooltipStyle);

    if (!window.OHC_TOOLTIPS) {
        window.OHC_TOOLTIPS = {};
        fetch("/api/tooltips").then(r => r.json()).then(data => { window.OHC_TOOLTIPS = data; }).catch(e => {
            // Fallback for playwright
            window.OHC_TOOLTIPS = {
                "nav-store": "Your Storefront. This is where you manage what you sell.",
                "nav-agents": "AI Helpers. These are your digital employees.",
                "ohc-help-btn": "Need help? Click here to access our Help Center and tutorials.",
                "search-input": "Search our knowledge base for help articles.",
                "dashboard-walkthrough-btn": "Take a tour of the dashboard",
                "api-docs-tooltip": "Direct API access is only for custom integrations."
            };
        });
    }

    const tooltipEl = document.createElement('div');
    tooltipEl.className = 'ohc-tooltip';
    document.body.appendChild(tooltipEl);

    function showTooltip(e, text) {
        if (!text) return;
        tooltipEl.textContent = text;
        const target = e.target.closest('[data-tooltip]') || e.target.closest('[id]');
        if (!target) return;
        const rect = target.getBoundingClientRect();

        let top = rect.top - tooltipEl.offsetHeight - 10;
        let left = rect.left + (rect.width / 2) - (tooltipEl.offsetWidth / 2);

        if (top < 10) {
            top = rect.bottom + 10; // show below
        }
        if (left < 10) {
            left = 10;
        }
        if (left + tooltipEl.offsetWidth > window.innerWidth - 10) {
            left = window.innerWidth - tooltipEl.offsetWidth - 10;
        }

        tooltipEl.style.top = top + 'px';
        tooltipEl.style.left = left + 'px';
        tooltipEl.classList.add('visible');
    }

    function hideTooltip() {
        tooltipEl.classList.remove('visible');
    }

    document.addEventListener('mouseover', (e) => {
        const target = e.target.closest('[data-tooltip]') || e.target.closest('[id]');
        if (target) {
            const text = (window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip');
            showTooltip(e, text);
        }
    });

    document.addEventListener('mouseout', (e) => {
        const target = e.target.closest('[data-tooltip]') || e.target.closest('[id]');
        if (target) {
            hideTooltip();
        }
    });

    // Mobile long press (contextmenu) or touchstart
    let touchTimeout;
    document.addEventListener('touchstart', (e) => {
        const target = e.target.closest('[data-tooltip]') || e.target.closest('[id]');
        if (target) {
            const text = (window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip');
            if (text) {
                touchTimeout = setTimeout(() => {
                    showTooltip(e, text);
                }, 500); // 500ms long press
            }
        }
    });

    document.addEventListener('touchend', () => {
        clearTimeout(touchTimeout);
        hideTooltip();
    });

    document.addEventListener('touchmove', () => {
        clearTimeout(touchTimeout);
        hideTooltip();
    });


    // --- HELP CHAT OVERLAY ---
    const chatStyle = document.createElement('style');
    chatStyle.textContent = `
        #ohc-help-btn {
            position: fixed;
            bottom: 20px;
            right: 20px;
            width: 56px;
            height: 56px;
            border-radius: 16px;
            background: #0066FF;
            color: white;
            border: none;
            box-shadow: 0 4px 14px rgba(0, 102, 255, 0.4);
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            z-index: 9999;
            transition: transform 0.2s;
        }
        #ohc-help-btn:hover {
            transform: scale(1.05);
        }
        #ohc-help-btn svg {
            width: 28px;
            height: 28px;
            fill: none;
            stroke: currentColor;
            stroke-width: 2;
        }
        #ohc-help-chat-overlay {
            position: fixed;
            bottom: 90px;
            right: 20px;
            width: 350px;
            height: 500px;
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%);
            border-radius: 16px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
            border: 1px solid rgba(0, 102, 255, 0.2);
            z-index: 10000;
            display: none;
            flex-direction: column;
            overflow: hidden;
            font-family: Outfit, sans-serif;
        }
        #ohc-help-chat-header {
            padding: 15px;
            background: rgba(0, 102, 255, 0.1);
            border-bottom: 1px solid rgba(0, 102, 255, 0.2);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        #ohc-help-chat-header h3 {
            margin: 0;
            font-size: 16px;
            color: #1d1d1f;
        }
        #ohc-help-close {
            background: none;
            border: none;
            cursor: pointer;
            color: #86868b;
        }
        #ohc-help-close:hover { color: #1d1d1f; }
        #ohc-help-messages {
            flex-grow: 1;
            padding: 15px;
            overflow-y: auto;
            display: flex;
            flex-direction: column;
            gap: 10px;
        }
        .msg {
            max-width: 80%;
            padding: 10px 15px;
            border-radius: 16px;
            font-size: 14px;
            line-height: 1.4;
        }
        .msg-ai {
            background: #e5e5ea;
            color: #1d1d1f;
            align-self: flex-start;
            border-bottom-left-radius: 4px;
        }
        .msg-user {
            background: #0066FF;
            color: white;
            align-self: flex-end;
            border-bottom-right-radius: 4px;
        }
        .msg a {
            color: #0066FF;
            text-decoration: underline;
            display: block;
            margin-top: 5px;
        }
        #ohc-help-input-area {
            padding: 15px;
            border-top: 1px solid rgba(0, 0, 0, 0.1);
            display: flex;
            gap: 10px;
            background: rgba(255, 255, 255, 0.5);
        }
        #ohc-help-input {
            flex-grow: 1;
            border: 1px solid rgba(0, 0, 0, 0.2);
            border-radius: 16px;
            padding: 10px 15px;
            outline: none;
            background: rgba(255, 255, 255, 0.8);
            font-size: 14px;
        }
        #ohc-help-input:focus { border-color: #0066FF; }
        #ohc-help-send {
            background: #0066FF;
            color: white;
            border: none;
            width: 40px;
            height: 40px;
            border-radius: 16px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
        }

        @media (prefers-color-scheme: dark) {
            #ohc-help-chat-overlay {
                background: rgba(22, 22, 26, 0.7);
                border-color: rgba(255, 255, 255, 0.1);
            }
            #ohc-help-chat-header, #ohc-help-input-area {
                background: rgba(0, 0, 0, 0.2);
                border-color: rgba(255, 255, 255, 0.1);
            }
            #ohc-help-chat-header h3 { color: #f5f5f7; }
            #ohc-help-input {
                background: rgba(0, 0, 0, 0.3);
                border-color: rgba(255, 255, 255, 0.2);
                color: white;
            }
        }
    `;
    document.head.appendChild(chatStyle);

    const btn = document.createElement('button');
    btn.id = 'ohc-help-btn';
    btn.setAttribute('aria-label', 'Open help chat');
    btn.innerHTML = `<svg viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>`;
    document.body.appendChild(btn);

    const overlay = document.createElement('div');
    overlay.id = 'ohc-help-chat-overlay';
    overlay.innerHTML = `
        <div id="ohc-help-chat-header">
            <h3>Ask AI Help</h3>
            <button id="ohc-help-close" aria-label="Close help chat">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path></svg>
            </button>
        </div>
        <div id="ohc-help-messages">
            <div class="msg msg-ai">Hi! I am your AI Help Agent! How can I assist you with OHC today?</div>
        </div>

        <div id="ohc-help-walkthroughs" style="margin-top: 15px; border-top: 1px solid rgba(0,0,0,0.1); padding-top: 15px;">
            <h4 style="margin: 0 0 10px 0; font-size: 14px;">Interactive Tours</h4>
            <button onclick="window.startWalkthrough && window.startWalkthrough([{targetId: '#nav-store', title: 'Set up your store', content: 'Click here to go to your storefront builder.'}])" style="width: 100%; text-align: left; padding: 8px; border: 1px solid rgba(0,0,0,0.1); border-radius: 8px; background: white; margin-bottom: 8px; cursor: pointer; font-size: 13px; color: black;">Tour: Set up your store</button>
            <button onclick="window.startWalkthrough && window.startWalkthrough([{targetId: '#nav-agents', title: 'Activate your AI Support Agent', content: 'Activate your AI agent to help with tasks.'}])" style="width: 100%; text-align: left; padding: 8px; border: 1px solid rgba(0,0,0,0.1); border-radius: 8px; background: white; margin-bottom: 8px; cursor: pointer; font-size: 13px; color: black;">Tour: Activate your AI Support Agent</button>
        </div>

        <div id="ohc-help-input-area">
            <input type="text" id="ohc-help-input" placeholder="Ask me anything..." />
            <button id="ohc-help-send" aria-label="Send message">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path></svg>
            </button>
        </div>
    `;
    document.body.appendChild(overlay);

    const closeBtn = document.getElementById('ohc-help-close');
    const messages = document.getElementById('ohc-help-messages');
    const input = document.getElementById('ohc-help-input');
    const sendBtn = document.getElementById('ohc-help-send');

    btn.addEventListener('click', () => {
        overlay.style.display = 'flex';
        input.focus();
    });

    closeBtn.addEventListener('click', () => {
        overlay.style.display = 'none';
    });

    async function sendMessage() {
        const text = input.value.trim();
        if (!text) return;

        // Add user msg
        const uMsg = document.createElement('div');
        uMsg.className = 'msg msg-user';
        uMsg.textContent = text;
        messages.appendChild(uMsg);
        input.value = '';
        messages.scrollTop = messages.scrollHeight;

        try {
            let url = '/api/chat';
            if (window.__TAURI__ && window.__TAURI__.core) {
                url = 'http://127.0.0.1:18789/api/chat';
            }
            const response = await fetch(url, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ message: text })
            });
            const data = await response.json();

            const aiMsg = document.createElement('div');
            aiMsg.className = 'msg msg-ai';
            if (data.link && data.link.url) {
                aiMsg.innerHTML = `${data.reply}<br/><a href="${data.link.url}">${data.link.title}</a>`;
            } else {
                aiMsg.textContent = data.reply;
            }
            messages.appendChild(aiMsg);
            messages.scrollTop = messages.scrollHeight;
        } catch (e) {
            console.error(e);
            const aiMsg = document.createElement('div');
            aiMsg.className = 'msg msg-ai';
            aiMsg.textContent = 'Sorry, there was an error connecting to the help assistant.';
            messages.appendChild(aiMsg);
            messages.scrollTop = messages.scrollHeight;
        }
    }

    sendBtn.addEventListener('click', sendMessage);
    input.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') sendMessage();
    });

    // --- WALKTHROUGHS ---
    const walkthroughStyle = document.createElement('style');
    walkthroughStyle.textContent = `
        .ohc-walkthrough-overlay {
            position: fixed;
            top: 0; left: 0; width: 100vw; height: 100vh;
            background: rgba(0, 0, 0, 0.4);
            backdrop-filter: blur(2px);
            z-index: 10000;
            pointer-events: auto;
        }
        .ohc-walkthrough-highlight {
            position: absolute;
            box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.6);
            border-radius: 8px;
            pointer-events: none;
            z-index: 10001;
            transition: all 0.3s ease;
        }
        .ohc-walkthrough-bubble {
            position: absolute;
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(20px) saturate(210%);
            border: 1px solid rgba(0, 102, 255, 0.3);
            border-radius: 16px;
            padding: 20px;
            width: 280px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
            z-index: 10002;
            font-family: Outfit, sans-serif;
            color: #1d1d1f;
        }
    `;
    document.head.appendChild(walkthroughStyle);

    window.startWalkthrough = function(steps) {
        if (!steps || steps.length === 0) return;

        let currentStep = 0;

        // Create overlay
        const wtOverlay = document.createElement('div');
        wtOverlay.id = 'walkthrough-overlay';
        wtOverlay.className = 'ohc-walkthrough-overlay fixed z-[90]';
        document.body.appendChild(wtOverlay);

        // Create bubble
        const bubble = document.createElement('div');
        bubble.id = 'walkthrough-bubble';
        bubble.className = 'ohc-walkthrough-bubble';
        bubble.setAttribute('role', 'dialog');
        document.body.appendChild(bubble);

        function renderStep() {
            const step = steps[currentStep];

            // Remove previous highlight
            document.querySelectorAll('.walkthrough-highlight').forEach(el => {
                el.classList.remove('walkthrough-highlight');
                el.style.position = '';
                el.style.zIndex = '';
                el.style.background = '';
            });

            // Fallback for steps using 'selector' instead of 'targetId'
            const targetIdOrSelector = step.targetId || step.selector;
            let target;
            if (targetIdOrSelector) {
                target = document.querySelector(targetIdOrSelector);
                if (!target && targetIdOrSelector.startsWith('#')) {
                    target = document.getElementById(targetIdOrSelector.substring(1));
                }
            }

            bubble.innerHTML = `
                <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #eee; padding-bottom: 8px; margin-bottom: 8px;">
                    <h4 style="margin: 0; font-size: 16px; font-weight: bold;">${step.title || 'Tour'}</h4>
                    <button class="ohc-walkthrough-close" id="wt-close" style="background: none; border: none; cursor: pointer; font-size: 18px;">&times;</button>
                </div>
                <p style="margin: 0; font-size: 14px; color: #333; line-height: 1.4;">${step.content || step.text}</p>
                <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 12px;">
                    ${currentStep > 0 ? '<button id="wt-prev" style="padding: 6px 12px; border: 1px solid #ccc; border-radius: 4px; background: white; cursor: pointer; font-size: 13px;">Back</button>' : ''}
                    <button id="wt-next" style="padding: 6px 12px; border: none; border-radius: 4px; background: #0066FF; color: white; cursor: pointer; font-size: 13px;">${currentStep === steps.length - 1 ? 'Finish' : 'Next'}</button>
                </div>
            `;

            document.getElementById('wt-close').onclick = closeWalkthrough;
            if (document.getElementById('wt-prev')) {
                document.getElementById('wt-prev').onclick = () => { currentStep--; renderStep(); };
            }
            document.getElementById('wt-next').onclick = () => {
                if (currentStep === steps.length - 1) {
                    closeWalkthrough();
                } else {
                    currentStep++;
                    renderStep();
                }
            };

            if (target) {
                target.classList.add('walkthrough-highlight');
                target.style.position = 'relative';
                target.style.zIndex = '99999';
                target.style.background = 'white'; // Make it pop

                const rect = target.getBoundingClientRect();
                // Position bubble below target if space permits, else above
                if (rect.bottom + 200 < window.innerHeight) {
                    bubble.style.top = (rect.bottom + 10) + 'px';
                } else {
                    bubble.style.top = Math.max(10, rect.top - bubble.offsetHeight - 10) + 'px';
                }
                bubble.style.left = Math.max(10, Math.min(rect.left, window.innerWidth - 320)) + 'px';
            } else {
                // Center if no target
                bubble.style.top = '50%';
                bubble.style.left = '50%';
                bubble.style.transform = 'translate(-50%, -50%)';
            }
        }

        function closeWalkthrough() {
            document.querySelectorAll('.walkthrough-highlight').forEach(el => {
                el.classList.remove('walkthrough-highlight');
                el.style.position = '';
                el.style.zIndex = '';
                el.style.background = '';
            });
            if (wtOverlay.parentNode) wtOverlay.parentNode.removeChild(wtOverlay);
            if (bubble.parentNode) bubble.parentNode.removeChild(bubble);
        }

        renderStep();
    };

});
