
// shared_docs.js
(function() {
    // Inject tooltip CSS
    const tooltipStyle = document.createElement('style');
    tooltipStyle.textContent = `
        .ohc-tooltip {
            position: fixed;
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(20px) saturate(210%);
            -webkit-backdrop-filter: blur(20px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.6);
            border-radius: 12px;
            padding: 12px 16px;
            color: #1d1d1f;
            font-family: "Outfit", -apple-system, BlinkMacSystemFont, sans-serif;
            font-size: 14px;
            line-height: 1.4;
            max-width: 250px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
            z-index: 10000;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.2s ease, transform 0.2s ease;
            transform: translateY(4px);
        }
        .ohc-tooltip.visible {
            opacity: 1;
            transform: translateY(0);
        }
    `;
    document.head.appendChild(tooltipStyle);

    // Initialize Tooltips
    document.addEventListener('DOMContentLoaded', () => {
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
                    "ohc-floating-help-btn": "Need help? Click here to access our Help Center and tutorials."
                };
            });
        }

        // Remove existing tooltip element if it exists to avoid duplicates
        const existingTooltip = document.querySelector('.ohc-tooltip');
        if (existingTooltip) {
            existingTooltip.remove();
        }

        const tooltipEl = document.createElement('div');
        tooltipEl.className = 'ohc-tooltip';
        document.body.appendChild(tooltipEl);

        function showTooltip(e, text) {
            if (!text) return;
            tooltipEl.textContent = text;
            const target = e.target.closest('[data-tooltip]') || e.target.closest('[id]');
            const targetRect = target ? target.getBoundingClientRect() : e.target.getBoundingClientRect();

            let left = targetRect.left + (targetRect.width / 2) - (tooltipEl.offsetWidth / 2);
            let top = targetRect.bottom + 10;

            if (left + tooltipEl.offsetWidth > window.innerWidth - 10) {
                left = window.innerWidth - tooltipEl.offsetWidth - 10;
            } else if (left < 10) {
                left = 10;
            }

            if (top + tooltipEl.offsetHeight > window.innerHeight - 10) {
                top = targetRect.top - tooltipEl.offsetHeight - 10;
            }

            tooltipEl.style.top = `${top}px`;
            tooltipEl.style.left = `${left}px`;
            tooltipEl.classList.add('visible');
        }

        function hideTooltip() {
            tooltipEl.classList.remove('visible');
        }

        let touchTimeout;

        document.addEventListener('mouseover', (e) => {
            const target = e.target.closest('[data-tooltip]') || e.target.closest('[id]');
            if (target) {
                const text = (window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip');
                if (text) {
                    showTooltip(e, text);
                }
            }
        });

        document.addEventListener('mouseout', (e) => {
            const target = e.target.closest('[data-tooltip]') || e.target.closest('[id]');
            if (target) {
                hideTooltip();
            }
        });

        document.addEventListener('touchstart', (e) => {
            const target = e.target.closest('[data-tooltip]') || e.target.closest('[id]');
            if (target) {
                touchTimeout = setTimeout(() => {
                    const text = (window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip');
                    if(text) {
                        showTooltip(e, text);
                    }
                }, 500); // 500ms long press
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
    });

    // Walkthrough Initialization
    window.startWalkthrough = function(steps) {
        if (!steps || steps.length === 0) return;

        let currentStep = 0;

        // Remove existing overlay/bubble
        const existingOverlay = document.getElementById('walkthrough-overlay');
        if (existingOverlay) existingOverlay.remove();
        const existingBubble = document.getElementById('walkthrough-bubble');
        if (existingBubble) existingBubble.remove();

        // Remove existing new-style classes
        document.querySelectorAll('.ohc-walkthrough-overlay').forEach(e => e.remove());
        document.querySelectorAll('.ohc-walkthrough-bubble').forEach(e => e.remove());

        // Create overlay
        const overlay = document.createElement('div');
        overlay.id = 'walkthrough-overlay';
        overlay.className = 'fixed z-[90] ohc-walkthrough-overlay';
        overlay.style.cssText = 'position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 99998;';
        document.body.appendChild(overlay);

        // Create bubble
        const bubble = document.createElement('div');
        bubble.id = 'walkthrough-bubble';
        bubble.className = 'ohc-walkthrough-bubble';
        bubble.setAttribute('role', 'dialog');
        bubble.style.cssText = 'position: fixed; background: rgba(255, 255, 255, 0.95); border-radius: 12px; padding: 20px; box-shadow: 0 10px 40px rgba(0,0,0,0.2); z-index: 99999; max-width: 300px; display: flex; flex-direction: column; gap: 12px; font-family: Outfit, sans-serif; backdrop-filter: blur(20px) saturate(210%); -webkit-backdrop-filter: blur(20px) saturate(210%); border: 1px solid rgba(255,255,255,0.8);';
        document.body.appendChild(bubble);

        function renderStep() {
            const step = steps[currentStep];

            // Clean up previous highlights
            document.querySelectorAll('.walkthrough-highlight').forEach(el => {
                el.classList.remove('walkthrough-highlight');
                el.style.position = '';
                el.style.zIndex = '';
                el.style.background = '';
                el.style.boxShadow = '';
                el.style.borderRadius = '';
            });

            // Find target (support selector or targetId)
            const target = document.querySelector(step.selector || step.targetId || '#' + step.targetId);

            bubble.innerHTML = `
                <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid rgba(0,0,0,0.05); padding-bottom: 12px;">
                    <h4 style="margin: 0; font-size: 16px; font-weight: 600; color: #1d1d1f;">${step.title || 'Tour'}</h4>
                    <button id="wt-close" class="ohc-walkthrough-close" aria-label="Close walkthrough step" style="background: none; border: none; cursor: pointer; font-size: 20px; color: #86868b; line-height: 1; padding: 0;">&times;</button>
                </div>
                <p style="margin: 0; font-size: 14px; color: #333; line-height: 1.5;">${step.content || step.text}</p>
                <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;">
                    ${currentStep > 0 ? '<button id="wt-prev" style="padding: 8px 16px; border: 1px solid #d2d2d7; border-radius: 8px; background: white; cursor: pointer; font-size: 13px; color: #1d1d1f; font-weight: 500;">Back</button>' : ''}
                    <button id="wt-next" style="padding: 8px 16px; border: none; border-radius: 8px; background: #0066FF; color: white; cursor: pointer; font-size: 13px; font-weight: 500;">${currentStep === steps.length - 1 ? 'Finish' : 'Next'}</button>
                </div>
            `;

            document.getElementById('wt-close').onclick = closeWalkthrough;
            if (document.getElementById('wt-prev')) document.getElementById('wt-prev').onclick = () => { currentStep--; renderStep(); };
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
                // Store original styles to restore later if needed, though class removal usually works
                target.style.position = 'relative';
                target.style.zIndex = '99999';
                target.style.background = 'white'; // Make it pop
                target.style.boxShadow = '0 0 0 4px rgba(0, 102, 255, 0.3)';
                target.style.borderRadius = getComputedStyle(target).borderRadius === '0px' ? '8px' : getComputedStyle(target).borderRadius;

                const rect = target.getBoundingClientRect();
                // Position bubble below target if space permits, else above
                if (rect.bottom + 200 < window.innerHeight) {
                    bubble.style.top = (rect.bottom + 16) + 'px';
                } else {
                    bubble.style.top = (rect.top - bubble.offsetHeight - 16) + 'px';
                }

                let proposedLeft = rect.left + (rect.width / 2) - (bubble.offsetWidth / 2);
                if (proposedLeft + bubble.offsetWidth > window.innerWidth - 10) {
                    proposedLeft = window.innerWidth - bubble.offsetWidth - 10;
                }
                if (proposedLeft < 10) {
                    proposedLeft = 10;
                }
                bubble.style.left = proposedLeft + 'px';
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
                el.style.boxShadow = '';
                el.style.borderRadius = '';
            });
            if (overlay.parentNode) overlay.parentNode.removeChild(overlay);
            if (bubble.parentNode) bubble.parentNode.removeChild(bubble);
        }

        renderStep();
    };

    // Chat Widget Initialization
    document.addEventListener('DOMContentLoaded', () => {
        // Prevent duplicate injection
        if (document.getElementById('ohc-help-btn') || document.getElementById('ohc-floating-help-btn')) return;

        // Inject chat CSS
        const chatStyle = document.createElement('style');
        chatStyle.textContent = `
            #ohc-help-btn {
                position: fixed;
                bottom: 24px;
                right: 24px;
                width: 56px;
                height: 56px;
                border-radius: 28px;
                background-color: rgba(0, 102, 255, 0.9);
                backdrop-filter: blur(10px) saturate(210%);
                -webkit-backdrop-filter: blur(10px) saturate(210%);
                color: white;
                border: 1px solid rgba(255,255,255,0.2);
                box-shadow: 0 4px 16px rgba(0, 102, 255, 0.3);
                cursor: pointer;
                z-index: 99990;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: transform 0.2s ease, box-shadow 0.2s ease;
            }
            #ohc-help-btn:hover {
                transform: scale(1.05);
                box-shadow: 0 6px 20px rgba(0, 102, 255, 0.4);
            }
            #ohc-help-btn svg {
                width: 28px;
                height: 28px;
                fill: currentColor;
            }
            #ohc-help-chat-overlay {
                position: fixed;
                bottom: 96px;
                right: 24px;
                width: 380px;
                height: 600px;
                max-height: calc(100vh - 120px);
                max-width: calc(100vw - 48px);
                background: rgba(255, 255, 255, 0.85);
                backdrop-filter: blur(30px) saturate(210%);
                -webkit-backdrop-filter: blur(30px) saturate(210%);
                border: 1px solid rgba(255, 255, 255, 0.6);
                border-radius: 16px;
                box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
                z-index: 99990;
                display: none;
                flex-direction: column;
                overflow: hidden;
                font-family: "Outfit", -apple-system, sans-serif;
                animation: slideUp 0.3s cubic-bezier(0.16, 1, 0.3, 1);
            }
            @keyframes slideUp {
                from { opacity: 0; transform: translateY(20px) scale(0.95); }
                to { opacity: 1; transform: translateY(0) scale(1); }
            }
            #ohc-help-chat-header {
                padding: 16px 20px;
                border-bottom: 1px solid rgba(0,0,0,0.05);
                display: flex;
                justify-content: space-between;
                align-items: center;
                background: rgba(255,255,255,0.5);
            }
            #ohc-help-chat-header h3 {
                margin: 0;
                font-size: 16px;
                font-weight: 600;
                color: #1d1d1f;
            }
            #ohc-help-close {
                background: none;
                border: none;
                color: #86868b;
                cursor: pointer;
                padding: 4px;
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: background 0.2s, color 0.2s;
            }
            #ohc-help-close:hover {
                background: rgba(0,0,0,0.05);
                color: #1d1d1f;
            }
            #ohc-help-messages {
                flex: 1;
                overflow-y: auto;
                padding: 20px;
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .msg {
                padding: 12px 16px;
                border-radius: 16px;
                font-size: 14px;
                line-height: 1.5;
                max-width: 85%;
                word-wrap: break-word;
            }
            .msg-ai {
                background: white;
                color: #1d1d1f;
                border: 1px solid rgba(0,0,0,0.05);
                box-shadow: 0 2px 8px rgba(0,0,0,0.02);
                border-bottom-left-radius: 4px;
            }
            .msg-user {
                background: #0066FF;
                color: white;
                align-self: flex-end;
                border-bottom-right-radius: 4px;
            }
            #ohc-help-walkthroughs {
                padding: 0 20px 20px 20px;
            }
            #ohc-help-walkthroughs button {
                width: 100%;
                text-align: left;
                padding: 12px 16px;
                border: 1px solid rgba(0,0,0,0.08);
                border-radius: 12px;
                background: white;
                margin-bottom: 8px;
                cursor: pointer;
                font-size: 13px;
                color: #1d1d1f;
                font-weight: 500;
                transition: border-color 0.2s, background 0.2s;
            }
            #ohc-help-walkthroughs button:hover {
                border-color: rgba(0, 102, 255, 0.3);
                background: #f8fbff;
            }
            #ohc-help-input-area {
                padding: 16px 20px;
                border-top: 1px solid rgba(0,0,0,0.05);
                display: flex;
                gap: 12px;
                background: rgba(255,255,255,0.5);
            }
            #ohc-help-input {
                flex: 1;
                padding: 10px 16px;
                border-radius: 20px;
                border: 1px solid #d2d2d7;
                background: white;
                font-size: 14px;
                outline: none;
                transition: border-color 0.2s;
                font-family: inherit;
            }
            #ohc-help-input:focus {
                border-color: #0066FF;
            }
            #ohc-help-send {
                width: 40px;
                height: 40px;
                border-radius: 20px;
                background: #0066FF;
                color: white;
                border: none;
                cursor: pointer;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: opacity 0.2s;
            }
            #ohc-help-send:hover {
                opacity: 0.9;
            }
            #ohc-help-send:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }

            @media (max-width: 480px) {
                #ohc-help-chat-overlay {
                    width: calc(100vw - 32px);
                    right: 16px;
                    bottom: 88px;
                }
                #ohc-help-btn {
                    right: 16px;
                    bottom: 16px;
                }
            }
        `;
        document.head.appendChild(chatStyle);

        // Inject HTML
        const btn = document.createElement('button');
        btn.id = 'ohc-help-btn';
        btn.setAttribute('aria-label', 'Open help chat');
        btn.title = 'Help';
        btn.innerHTML = `<svg viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z"/></svg>`;
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

            <div id="ohc-help-walkthroughs">
                <h4 style="margin: 0 0 12px 0; font-size: 13px; color: #86868b; text-transform: uppercase; letter-spacing: 0.5px;">Interactive Tours</h4>
                <button onclick="window.startWalkthrough && window.startWalkthrough([{targetId: 'nav-store', title: 'Set up your store', content: 'Click here to access your storefront and add your first products.'}])">Tour: Set up your store</button>
                <button onclick="window.startWalkthrough && window.startWalkthrough([{targetId: 'nav-settings', title: 'Accept your first payment', content: 'Go to Settings > Payments to connect your bank account.'}])">Tour: Accept your first payment</button>
                <button onclick="window.startWalkthrough && window.startWalkthrough([{targetId: 'nav-agents', title: 'Activate your AI Support Agent', content: 'Visit the AI Agents tab to hire your first digital assistant.'}])">Tour: Activate your AI Support Agent</button>
                <div style="margin-top: 12px; text-align: center;">
                    <a href="/api/ui/help.html" style="color: #0066FF; text-decoration: none; font-size: 13px; font-weight: 500;">Go to full Help Center &rarr;</a>
                </div>
            </div>

            <div id="ohc-help-input-area">
                <input type="text" id="ohc-help-input" placeholder="Ask anything..." />
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
            overlay.style.display = overlay.style.display === 'flex' ? 'none' : 'flex';
            if (overlay.style.display === 'flex') {
                input.focus();
            }
        });

        closeBtn.addEventListener('click', () => {
            overlay.style.display = 'none';
        });

        function addMsg(text, type) {
            const div = document.createElement('div');
            div.className = 'msg msg-' + type;
            div.textContent = text;
            messages.appendChild(div);
            messages.scrollTop = messages.scrollHeight;
        }

        async function handleSend() {
            const text = input.value.trim();
            if (!text) return;

            addMsg(text, 'user');
            input.value = '';
            input.disabled = true;
            sendBtn.disabled = true;

            try {
                let articles = [];
                if (window.__TAURI__ && window.__TAURI__.core) {
                    articles = await window.__TAURI__.core.invoke('get_help_articles', {});
                } else {
                    articles = await fetch('/api/help').then(r => r.json()).catch(() => []);
                }

                // Simulate AI finding relevant article
                const lowerText = text.toLowerCase();
                const matched = articles.find(a =>
                    a.title.toLowerCase().includes(lowerText) ||
                    a.desc.toLowerCase().includes(lowerText) ||
                    lowerText.includes(a.title.toLowerCase())
                );

                let aiResponse = "I can help with that. ";
                if (matched) {
                     aiResponse += `I found an article that might help: ${matched.title}. Read the full article -> ${matched.link}`;
                } else {
                     aiResponse += "Based on our docs, you can usually find this in your settings or dashboard. Can you provide more details?";
                }

                addMsg(aiResponse, 'ai');
            } catch (err) {
                console.error(err);
                addMsg("I'm sorry, I'm having trouble connecting right now. Please try again later.", 'ai');
            } finally {
                input.disabled = false;
                sendBtn.disabled = false;
                input.focus();
            }
        }

        sendBtn.addEventListener('click', handleSend);
        input.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') handleSend();
        });
    });

})();
