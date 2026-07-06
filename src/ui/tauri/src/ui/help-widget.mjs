document.addEventListener('DOMContentLoaded', () => {
// Inject floating widget styles
    const style = document.createElement('style');
    style.textContent = \`
        #ohc-floating-help-btn {
            position: fixed;
            bottom: 24px;
            right: 24px;
            width: 56px;
            height: 56px;
            border-radius: 28px;
            background: rgba(255, 255, 255, 0.2);
            backdrop-filter: blur(40px) saturate(200%);
            -webkit-backdrop-filter: blur(40px) saturate(200%);
            color: #0f172a;
            border: 1px solid rgba(255, 255, 255, 0.5);
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
            cursor: pointer;
            z-index: 99990;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.2s cubic-bezier(0.25, 0.8, 0.25, 1);
        }
        #ohc-floating-help-btn:hover {
            transform: scale(1.05);
            background: rgba(255, 255, 255, 0.3);
            box-shadow: 0 12px 48px rgba(0, 0, 0, 0.15);
        }
        #ohc-floating-help-btn svg {
            width: 28px;
            height: 28px;
            fill: currentColor;
        }
        #ai-chat-interface {
            position: fixed;
            bottom: 96px;
            right: 24px;
            width: 380px;
            height: 600px;
            max-height: calc(100vh - 120px);
            background: rgba(255, 255, 255, 0.7);
            backdrop-filter: blur(40px) saturate(200%);
            -webkit-backdrop-filter: blur(40px) saturate(200%);
            border-radius: 20px;
            box-shadow: 0 12px 48px rgba(0, 0, 0, 0.15);
            z-index: 99990;
            display: none;
            flex-direction: column;
            overflow: hidden;
            font-family: Outfit, -apple-system, sans-serif;
            border: 1px solid rgba(255, 255, 255, 0.6);
        }

        /* Tooltip Styles */
        .ohc-tooltip {
            position: fixed;
            background: rgba(255, 255, 255, 0.7);
            backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.5);
            color: #0f172a;
            padding: 8px 12px;
            border-radius: 16px;
            font-size: 13px;
            font-family: Outfit, sans-serif;
            z-index: 100000;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.2s;
            box-shadow: 0 4px 12px rgba(0,0,0,0.15);
            max-width: 250px;
            line-height: 1.4;
        }
        .ohc-tooltip.visible {
            opacity: 1;
        }

        @media (max-width: 480px) {
            #ai-chat-interface {
                bottom: 0;
                right: 0;
                width: 100vw;
                height: 100%;
                max-height: 100vh;
                border-radius: 0;
                box-sizing: border-box;
            }
        }
        #ohc-floating-help-header {
            padding: 16px;
            background: rgba(248, 250, 252, 0.6);
            border-bottom: 1px solid rgba(226, 232, 240, 0.5);
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        #ohc-floating-help-header h3 {
            margin: 0;
            font-size: 18px;
            color: #0f172a;
            font-weight: 600;
        }
        #ohc-floating-help-close {
            background: none;
            border: none;
            font-size: 24px;
            color: #64748b;
            cursor: pointer;
            padding: 0 4px;
            line-height: 1;
        }
        #ohc-floating-help-tabs {
            display: flex;
            border-bottom: 1px solid rgba(226, 232, 240, 0.5);
            background: rgba(255, 255, 255, 0.4);
        }
        .ohc-help-tab {
            flex: 1;
            padding: 12px 0;
            text-align: center;
            background: none;
            border: none;
            font-size: 13px;
            font-weight: 600;
            color: #64748b;
            cursor: pointer;
            border-bottom: 2px solid transparent;
            font-family: inherit;
        }
        .ohc-help-tab.active {
            color: #0066FF;
            border-bottom-color: #0066FF;
        }
        .ohc-help-content {
            display: none;
            flex: 1;
            overflow-y: auto;
            padding: 16px;
            flex-direction: column;
        }
        .ohc-help-content.active {
            display: flex;
        }
        .ohc-chat-messages {
            flex: 1;
            overflow-y: auto;
            display: flex;
            flex-direction: column;
            gap: 12px;
            margin-bottom: 16px;
        }
        .ohc-chat-message {
            max-width: 85%;
            padding: 12px 16px;
            border-radius: 16px;
            font-size: 14px;
            line-height: 1.5;
        }
        .ohc-chat-message.user {
            align-self: flex-end;
            background: #0066FF;
            color: white;
            border-bottom-right-radius: 4px;
        }
        .ohc-chat-message.agent {
            align-self: flex-start;
            background: rgba(241, 245, 249, 0.8);
            color: #0f172a;
            border-bottom-left-radius: 4px;
            border: 1px solid rgba(226, 232, 240, 0.5);
        }
        .ohc-chat-message a {
            color: #0066FF;
            text-decoration: underline;
            display: block;
            margin-top: 8px;
            font-weight: 500;
        }
        .ohc-chat-input-area {
            display: flex;
            gap: 8px;
            background: rgba(255, 255, 255, 0.5);
            padding: 8px;
            border-radius: 24px;
            border: 1px solid rgba(226, 232, 240, 0.8);
        }
        #ohc-help-chat-input {
            flex: 1;
            border: none;
            background: none;
            padding: 8px 12px;
            font-size: 14px;
            font-family: inherit;
            outline: none;
        }
        #ohc-help-chat-send {
            background: #0066FF;
            color: white;
            border: none;
            border-radius: 16px;
            padding: 0 16px;
            font-weight: 600;
            cursor: pointer;
            transition: background 0.2s;
        }
        #ohc-help-chat-send:disabled {
            background: #cbd5e1;
            cursor: not-allowed;
        }
        .ohc-tour-card {
            background: rgba(255, 255, 255, 0.5);
            border: 1px solid rgba(226, 232, 240, 0.8);
            border-radius: 12px;
            padding: 16px;
            margin-bottom: 12px;
            cursor: pointer;
            transition: all 0.2s;
        }
        .ohc-tour-card:hover {
            background: rgba(255, 255, 255, 0.8);
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0,0,0,0.05);
            border-color: #0066FF;
        }
        .ohc-tour-card h4 {
            margin: 0 0 4px 0;
            color: #0f172a;
            font-size: 15px;
        }
        .ohc-tour-card p {
            margin: 0;
            color: #64748b;
            font-size: 13px;
        }
        .ohc-video-card {
            display: flex;
            gap: 12px;
            background: rgba(255, 255, 255, 0.5);
            border: 1px solid rgba(226, 232, 240, 0.8);
            border-radius: 12px;
            padding: 12px;
            cursor: pointer;
            align-items: center;
        }
        .ohc-video-card:hover {
            background: rgba(255, 255, 255, 0.8);
        }
        .ohc-video-thumb {
            width: 80px;
            height: 45px;
            background: #cbd5e1;
            border-radius: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .ohc-video-info h4 {
            margin: 0 0 2px 0;
            font-size: 13px;
            color: #0f172a;
            display: -webkit-box;
            -webkit-line-clamp: 2;
            -webkit-box-orient: vertical;
            overflow: hidden;
        }
        .ohc-video-info p {
            margin: 0;
            font-size: 11px;
            color: #64748b;
        }
    \`;
    document.head.appendChild(style);

    // Create the button
    const btn = document.createElement('button');
    btn.id = 'ohc-floating-help-btn';
    btn.setAttribute('aria-label', 'Help');
    btn.title = 'Help';
    btn.innerHTML = \`<svg viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z"/></svg>\`;
    document.body.appendChild(btn);

    // Create the widget
    const widget = document.createElement('div');
    widget.id = 'ai-chat-interface';
    widget.innerHTML = \`
        <div id="ohc-floating-help-header">
            <h3>OHC Help</h3>
            <button id="ohc-floating-help-close">&times;</button>
        </div>
        <div id="ohc-floating-help-tabs">
            <button class="ohc-help-tab active" data-target="tab-articles">Articles</button>
            <button class="ohc-help-tab" data-target="tab-tours">Interactive Tours</button>
            <button class="ohc-help-tab" data-target="tab-videos">Videos</button>
            <button class="ohc-help-tab" data-target="tab-chat" aria-label="Ask AI">Ask AI</button>
        </div>

        <div id="tab-articles" class="ohc-help-content active">
            <div style="margin-bottom: 16px;">
                <a href="/help.html" style="display: block; padding: 12px; background: rgba(241, 245, 249, 0.8); border-radius: 8px; text-decoration: none; color: #0f172a; font-weight: 500; text-align: center;">Open Full Help Center</a>
            </div>
            <h4>Popular Articles</h4>
            <ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 8px;" id="ohc-help-articles-list">
                <li><a href="/help_article.html?id=getting-started-1" style="color: #0066FF; text-decoration: none; font-size: 14px;">Welcome to One Human Corp</a></li>
                <li><a href="/help_article.html?id=my-store-1" style="color: #0066FF; text-decoration: none; font-size: 14px;">Setting up your storefront</a></li>
                <li><a href="/help_article.html?id=payments-1" style="color: #0066FF; text-decoration: none; font-size: 14px;">Accepting your first payment</a></li>
            </ul>
            <div style="margin-top: auto; padding-top: 16px; border-top: 1px solid rgba(226, 232, 240, 0.5);">
                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px;">
                  <label style="font-size: 14px; font-weight: 500; color: #0f172a;">Advanced Controls</label>
                  <label class="switch" style="position: relative; display: inline-block; width: 36px; height: 20px;">
                    <input type="checkbox" id="advanced-toggle-help" style="opacity: 0; width: 0; height: 0;">
                    <span class="slider" style="position: absolute; cursor: pointer; top: 0; left: 0; right: 0; bottom: 0; background-color: #ccc; transition: .4s; border-radius: 20px;"></span>
                  </label>
                </div>
                <div id="advanced-help-links" style="display: none;">
                    <a href="/api-docs.html" style="color: #64748b; font-size: 13px; text-decoration: none; display: block; margin-bottom: 8px;">OHC Advanced API Reference</a>
                    <a href="/tooltip-registry.html" style="color: #64748b; font-size: 13px; text-decoration: none; display: block;">Tooltip Registry</a>
                </div>
            </div>
        </div>

        <div id="tab-tours" class="ohc-help-content">
            <div class="ohc-tour-card" onclick="window.startWalkthrough && window.startWalkthrough([{targetId: '#nav-store', title: 'Set up your store', content: 'Click here to access your storefront and add your first products.'}])">
                <h4>Set up your store</h4>
                <p>Learn how to add products and customize your storefront.</p>
            </div>
            <div class="ohc-tour-card" onclick="window.startWalkthrough && window.startWalkthrough([{targetId: '#nav-settings', title: 'Accept your first payment', content: 'Go to Settings > Payments to connect your bank account.'}])">
                <h4>Accept your first payment</h4>
                <p>Connect your account to start receiving money.</p>
            </div>
            <div class="ohc-tour-card" onclick="window.startWalkthrough && window.startWalkthrough([{targetId: '#nav-agents', title: 'Activate your AI Support Agent', content: 'Visit the AI Agents tab to hire your first digital assistant.'}])">
                <h4>Activate your AI Support Agent</h4>
                <p>Let AI handle customer queries for you.</p>
            </div>
        </div>

        <div id="tab-videos" class="ohc-help-content">
            <div id="video-list" style="display: flex; flex-direction: column; gap: 12px;">Loading videos...</div>
        </div>

        <div id="tab-chat" class="ohc-help-content" style="padding-bottom: 12px;">
            <div class="ohc-chat-messages" id="ohc-help-chat-messages">
                <div class="ohc-chat-message agent">Hi! I'm your OHC support agent. How can I help you today?</div>
            </div>
            <div class="ohc-chat-input-area">
                <input type="text" id="ohc-help-chat-input" placeholder="Ask anything...">
                <button id="ohc-help-chat-send" disabled>Send</button>
            </div>
        </div>
    \`;
    document.body.appendChild(widget);

    // Advanced toggle
    const advToggle = document.getElementById('advanced-toggle-help');
    const advLinks = document.getElementById('advanced-help-links');
    if (advToggle && advLinks) {
        advToggle.addEventListener('change', (e) => {
            advLinks.style.display = e.target.checked ? 'block' : 'none';
        });
        // Add styling for the slider inline to ensure it works
        const s = document.createElement('style');
        s.textContent = \`
            #advanced-toggle-help:checked + .slider { background-color: #0066FF; }
            .slider:before { position: absolute; content: ""; height: 16px; width: 16px; left: 2px; bottom: 2px; background-color: white; transition: .4s; border-radius: 50%; }
            #advanced-toggle-help:checked + .slider:before { transform: translateX(16px); }
        \`;
        document.head.appendChild(s);
    }

    // Toggle widget
    btn.addEventListener('click', () => {
        const isVisible = widget.style.display === 'flex';
        widget.style.display = isVisible ? 'none' : 'flex';
        if (!isVisible) {
            document.getElementById('ohc-help-chat-input').focus();
            loadVideos();
        }
    });

    document.getElementById('ohc-floating-help-close').addEventListener('click', () => {
        widget.style.display = 'none';
    });

    // Tab switching
    const tabs = document.querySelectorAll('.ohc-help-tab');
    const contents = document.querySelectorAll('.ohc-help-content');
    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            tabs.forEach(t => t.classList.remove('active'));
            contents.forEach(c => c.classList.remove('active'));
            tab.classList.add('active');
            document.getElementById(tab.getAttribute('data-target')).classList.add('active');
        });
    });

    // Load videos dynamically
    let videosLoaded = false;
    async function loadVideos() {
        if (videosLoaded) return;
        const videoList = document.getElementById('video-list');
        try {
            let data = [];
            if (window.__TAURI__ && window.__TAURI__.core) {
                data = await window.__TAURI__.core.invoke('get_help_videos', {});
            } else {
                const res = await fetch('/api/videos');
                data = await res.json();
            }
            videoList.innerHTML = '';
            data.slice(0, 3).forEach(v => {
                videoList.innerHTML += \`
                    <div class="ohc-video-card" onclick="window.location.href='/help.html?play=\${v.id}'">
                        <div class="ohc-video-thumb">
                            <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
                        </div>
                        <div class="ohc-video-info">
                            <h4>\${v.title}</h4>
                            <p>\${v.duration}</p>
                        </div>
                    </div>
                \`;
            });
            videoList.innerHTML += \`<a href="/help.html" style="text-align: center; color: #0066FF; font-size: 13px; text-decoration: none; margin-top: 8px;">View all videos →</a>\`;
            videosLoaded = true;
        } catch (e) {
            videoList.innerHTML = 'Failed to load videos.';
        }
    }

    // Chat functionality
    const chatInput = document.getElementById('ohc-help-chat-input');
    const chatSend = document.getElementById('ohc-help-chat-send');
    const chatMessages = document.getElementById('ohc-help-chat-messages');

    function appendMessage(text, sender, link = null) {
        const msg = document.createElement('div');
        msg.className = \`ohc-chat-message \${sender}\`;
        msg.textContent = text;
        if (link && link.url && link.title) {
            msg.innerHTML += \`<a href="\${link.url}">\${link.title}</a>\`;
        }
        chatMessages.appendChild(msg);
        chatMessages.scrollTop = chatMessages.scrollHeight;
    }

    async function handleSend() {
        const text = chatInput.value.trim();
        if (!text) return;

        appendMessage(text, 'user');
        chatInput.value = '';

        try {
            const res = await fetch('/api/chat', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ message: text })
            });
            const data = await res.json();

            if (data.reply) {
                appendMessage(data.reply.text || data.reply, 'agent', data.reply.link || data.link);
            } else if (data.text) {
                appendMessage(data.text, 'agent', data.link);
            } else {
                appendMessage(data, 'agent');
            }
        } catch (e) {
            appendMessage("I'm sorry, I'm having trouble connecting right now.", 'agent');
        }
    }

    chatSend.addEventListener('click', handleSend);
    chatInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter' && !chatSend.disabled) handleSend();
    });
    chatInput.addEventListener('input', (e) => {
        chatSend.disabled = e.target.value.trim() === '';
    });

    // --- Global Tooltip & Walkthrough Logic ---

    // Tooltips
    if (!window.OHC_TOOLTIPS) {
        window.OHC_TOOLTIPS = {};
        fetch("/api/tooltips").then(r => r.json()).then(data => { window.OHC_TOOLTIPS = data; }).catch(e => {
            // Silently fail if tooltips API is unavailable
        });
    }
    const tooltipEl = document.createElement('div');
    tooltipEl.className = 'ohc-tooltip';
    document.body.appendChild(tooltipEl);

    function showTooltip(e, text) {
        if (!text) return;
        tooltipEl.textContent = text;
        const targetRect = e.target.closest('[data-tooltip], [id]') ? e.target.closest('[data-tooltip], [id]').getBoundingClientRect() : e.target.getBoundingClientRect();

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

        tooltipEl.style.top = \`\${top}px\`;
        tooltipEl.style.left = \`\${left}px\`;
        tooltipEl.classList.add('visible');
    }

    function hideTooltip() {
        tooltipEl.classList.remove('visible');
    }

    document.addEventListener('mouseover', (e) => {
        const target = e.target.closest('[data-tooltip], [id]');
        if (target) {
            const text = (window.OHC_TOOLTIPS && target.id && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip');
            if (text) {
                showTooltip(e, text);
            }
        }
    });

    document.addEventListener('mouseout', (e) => {
        const target = e.target.closest('[data-tooltip], [id]');
        if (target) {
            hideTooltip();
        }
    });


    document.addEventListener('touchstart', (e) => {
        const target = e.target.closest('[id], [data-tooltip]');
        if (target) {
            const text = (window.OHC_TOOLTIPS && target.id && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip');
            if (text) {
                window.touchTimer = setTimeout(() => {
                    showTooltip(e.touches ? e.touches[0] : e, text);
                }, 500); // 500ms long press
            }
        }
    });

    document.addEventListener('touchend', (e) => {
        clearTimeout(window.touchTimer);
        hideTooltip();
    });

    document.addEventListener('touchmove', (e) => {
        clearTimeout(window.touchTimer);
        hideTooltip();
    });

    document.addEventListener('touchcancel', (e) => {
        clearTimeout(window.touchTimer);
        hideTooltip();
    });

    // Walkthroughs
    if (!window.startWalkthrough) {
        window.startWalkthrough = function(steps) {
            if (!steps || steps.length === 0) return;

            let currentStep = 0;

            const overlay = document.createElement('div');
            overlay.id = 'walkthrough-overlay'; overlay.classList.add('ohc-walkthrough-overlay');
            overlay.style.cssText = 'position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 99998;';
            document.body.appendChild(overlay);

            const bubble = document.createElement('div');
            bubble.id = 'walkthrough-bubble'; bubble.classList.add('ohc-walkthrough-bubble');
            bubble.setAttribute('role', 'dialog');
            bubble.style.cssText = 'position: fixed; z-index: 99999; max-width: 300px; display: flex; flex-direction: column; gap: 8px; font-family: Outfit, sans-serif; padding: 16px; border-radius: 16px;';
            bubble.classList.add('glassmorphism');
            document.body.appendChild(bubble);

            function renderStep() {
                const step = steps[currentStep]; if (typeof bubbleEl !== "undefined" && bubbleEl) { bubbleEl.setAttribute("aria-label", (step.title || "Tour") + " walkthrough step"); } else if (typeof bubble !== "undefined" && bubble) { bubble.setAttribute("aria-label", (step.title || "Tour") + " walkthrough step"); } bubble.setAttribute('aria-label', (step.title || 'Tour') + ' walkthrough step');

                document.querySelectorAll('.walkthrough-highlight, .ohc-walkthrough-highlight').forEach(el => {
                    el.classList.remove('walkthrough-highlight', 'ohc-walkthrough-highlight', 'glassmorphism');
                    el.style.position = '';
                    el.style.zIndex = '';
                el.style.pointerEvents = '';
                });

                const target = document.getElementById(step.targetId) || document.querySelector(step.targetId || step.selector);

                bubble.innerHTML = \`
                    <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #eee; padding-bottom: 8px; margin-bottom: 8px;">
                        <h4 style="margin: 0; font-size: 16px; font-weight: bold;">\${step.title || 'Tour'}</h4>
                        <button id="wt-close" class="ohc-walkthrough-close" aria-label="Close walkthrough" style="background: none; border: none; cursor: pointer; font-size: 18px;">&times;</button>
                    </div>
                    <p style="margin: 0; font-size: 14px; color: #333;">\${step.content || step.text}</p>
                    <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;">
                        \${currentStep > 0 ? '<button id="wt-prev" class="glassmorphism" style="min-height: 44px; min-width: 80px; display: inline-flex; align-items: center; justify-content: center; padding: 6px 12px; border-radius: 8px; cursor: pointer;">Back</button>' : ''}
                        <button id="wt-next" style="min-height: 44px; display: inline-flex; align-items: center; justify-content: center; padding: 6px 12px; border: none; border-radius: 8px; background: #2563eb; color: white; cursor: pointer;">\${currentStep === steps.length - 1 ? 'Finish' : 'Next'}</button>
                    </div>
                \`;

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
                    target.classList.add('glassmorphism');
                    target.style.position = 'relative';
                    target.style.zIndex = '99999';
                    target.style.pointerEvents = 'none';

                    const rect = target.getBoundingClientRect();
                    if (rect.bottom + 200 < window.innerHeight) {
                        bubble.style.top = (rect.bottom + 10) + 'px';
                    } else {
                        bubble.style.top = (rect.top - bubble.offsetHeight - 10) + 'px';
                    }
                    bubble.style.left = Math.max(10, Math.min(rect.left, window.innerWidth - 320)) + 'px';
                } else {
                    bubble.style.top = '50%';
                    bubble.style.left = '50%';
                    bubble.style.transform = 'translate(-50%, -50%)';
                }
            }

            function closeWalkthrough() {
                document.querySelectorAll('.walkthrough-highlight, .ohc-walkthrough-highlight').forEach(el => {
                    el.classList.remove('walkthrough-highlight', 'ohc-walkthrough-highlight', 'glassmorphism');
                    el.style.position = '';
                    el.style.zIndex = '';
                el.style.pointerEvents = '';
                });
                if (overlay.parentNode) overlay.parentNode.removeChild(overlay);
                if (bubble.parentNode) bubble.parentNode.removeChild(bubble);
            }

            renderStep();
        };
    }
});

// Help widget logic globally initialized.
