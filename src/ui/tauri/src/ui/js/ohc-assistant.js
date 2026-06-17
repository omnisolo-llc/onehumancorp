document.addEventListener('DOMContentLoaded', () => {
    // Inject CSS
    if (!document.getElementById('ohc-assistant-css')) {
        const style = document.createElement('style');
        style.id = 'ohc-assistant-css';
        style.innerHTML = `
            .ohc-tooltip {
                position: fixed;
                background: rgba(255, 255, 255, 0.7);
                backdrop-filter: blur(30px) saturate(210%);
                -webkit-backdrop-filter: blur(30px) saturate(210%);
                border: 1px solid rgba(255, 255, 255, 0.5);
                color: #1d1d1f;
                padding: 10px 14px;
                border-radius: 8px;
                font-size: 13px;
                box-shadow: 0 4px 12px rgba(0,0,0,0.15);
                z-index: 99999;
                pointer-events: none;
                opacity: 0;
                transition: opacity 0.2s ease-in-out;
                max-width: 250px;
                line-height: 1.4;
                font-family: "Outfit", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            }
            .ohc-tooltip.visible {
                opacity: 1;
            }
            /* Chat styles */
            #ohc-help-chat-widget {
                position: fixed;
                bottom: 20px;
                right: 20px;
                z-index: 99998;
                font-family: "Outfit", -apple-system, sans-serif;
            }
            #ohc-help-chat-btn {
                width: 60px;
                height: 60px;
                border-radius: 30px;
                background-color: #0066FF;
                color: white;
                border: none;
                box-shadow: 0 4px 12px rgba(0,0,0,0.2);
                cursor: pointer;
                font-size: 24px;
                display: flex;
                align-items: center;
                justify-content: center;
                transition: transform 0.2s;
            }
            #ohc-help-chat-btn:hover {
                transform: scale(1.05);
            }
            #ohc-help-chat-window {
                display: none;
                position: absolute;
                bottom: 80px;
                right: 0;
                width: 320px;
                height: 400px;
                background: rgba(255, 255, 255, 0.9);
                backdrop-filter: blur(30px) saturate(210%);
                -webkit-backdrop-filter: blur(30px) saturate(210%);
                border: 1px solid rgba(255, 255, 255, 0.5);
                border-radius: 16px;
                box-shadow: 0 10px 30px rgba(0,0,0,0.15);
                flex-direction: column;
                overflow: hidden;
            }
            #ohc-help-chat-header {
                padding: 16px;
                background: rgba(255, 255, 255, 0.5);
                border-bottom: 1px solid rgba(0,0,0,0.1);
                display: flex;
                justify-content: space-between;
                align-items: center;
            }
            #ohc-help-chat-header h3 {
                margin: 0;
                font-size: 16px;
                color: #1d1d1f;
            }
            #ohc-help-chat-close {
                background: none;
                border: none;
                font-size: 20px;
                cursor: pointer;
                color: #86868b;
            }
            #ohc-help-chat-messages {
                flex: 1;
                padding: 16px;
                overflow-y: auto;
                display: flex;
                flex-direction: column;
                gap: 12px;
            }
            .ohc-chat-msg {
                max-width: 80%;
                padding: 10px 14px;
                border-radius: 12px;
                font-size: 14px;
                line-height: 1.4;
            }
            .ohc-chat-msg.agent {
                background: #f5f5f7;
                color: #1d1d1f;
                align-self: flex-start;
                border-bottom-left-radius: 4px;
            }
            .ohc-chat-msg.user {
                background: #0066FF;
                color: white;
                align-self: flex-end;
                border-bottom-right-radius: 4px;
            }
            .ohc-chat-msg a {
                color: #0066FF;
                text-decoration: none;
                display: block;
                margin-top: 8px;
                font-weight: 500;
            }
            .ohc-chat-msg.user a {
                color: white;
            }
            #ohc-help-chat-input-area {
                padding: 12px;
                border-top: 1px solid rgba(0,0,0,0.1);
                display: flex;
                gap: 8px;
            }
            #ohc-help-chat-input {
                flex: 1;
                padding: 10px;
                border: 1px solid #d2d2d7;
                border-radius: 20px;
                font-family: inherit;
                font-size: 14px;
            }
            #ohc-help-chat-send {
                background: #0066FF;
                color: white;
                border: none;
                border-radius: 20px;
                padding: 0 16px;
                cursor: pointer;
                font-weight: 500;
            }
        `;
        document.head.appendChild(style);
    }

    // --- Global Tooltip Logic ---

    // Load tooltips if not already loaded
    if (!window.OHC_TOOLTIPS) {
        window.OHC_TOOLTIPS = {};
        fetch("/api/tooltips").then(r => r.json()).then(data => { window.OHC_TOOLTIPS = data; }).catch(e => {
            console.error("Failed to load tooltips", e);
        });
    }

    const tooltipEl = document.createElement('div');
    tooltipEl.className = 'ohc-tooltip';
    document.body.appendChild(tooltipEl);

    function showTooltip(e, text) {
        if (!text) return;
        tooltipEl.textContent = text;
        const targetRect = e.target.closest('[id]') ? e.target.closest('[id]').getBoundingClientRect() : e.target.getBoundingClientRect();

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

    document.addEventListener('touchcancel', (e) => {
        clearTimeout(window.touchTimer);
        hideTooltip();
    });

    // --- Walkthrough Logic ---
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
            bubble.style.cssText = 'position: fixed; background: rgba(255, 255, 255, 0.7); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.5); padding: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.15); z-index: 99999; max-width: 320px; display: flex; flex-direction: column; gap: 12px; font-family: "Outfit", sans-serif;';
            document.body.appendChild(bubble);

            function renderStep() {
                const step = steps[currentStep];

                document.querySelectorAll('.walkthrough-highlight, .ohc-walkthrough-highlight').forEach(el => {
                    el.classList.remove('walkthrough-highlight', 'ohc-walkthrough-highlight');
                    el.style.position = '';
                    el.style.zIndex = '';
                    el.style.background = '';
                    el.style.pointerEvents = '';
                });

                const target = document.getElementById(step.targetId) || document.querySelector(step.targetId || step.selector);

                bubble.innerHTML = \`
                    <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid rgba(0,0,0,0.1); padding-bottom: 12px; margin-bottom: 8px;">
                        <h4 style="margin: 0; font-size: 18px; font-weight: 600; color: #1d1d1f;">\${step.title || 'Tour'}</h4>
                        <button id="wt-close" class="ohc-walkthrough-close" aria-label="Close walkthrough" style="background: none; border: none; cursor: pointer; font-size: 24px; color: #86868b; line-height: 1;">&times;</button>
                    </div>
                    <p style="margin: 0; font-size: 15px; color: #1d1d1f; line-height: 1.5;">\${step.content || step.text}</p>
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 12px;">
                        <span style="font-size: 13px; color: #86868b;">Step \${currentStep + 1} of \${steps.length}</span>
                        <div style="display: flex; gap: 8px;">
                            \${currentStep > 0 ? '<button id="wt-prev" style="padding: 8px 16px; border: 1px solid #d2d2d7; border-radius: 20px; background: white; cursor: pointer; font-weight: 500; color: #1d1d1f;">Back</button>' : ''}
                            <button id="wt-next" style="padding: 8px 16px; border: none; border-radius: 20px; background: #0066FF; color: white; cursor: pointer; font-weight: 500;">\${currentStep === steps.length - 1 ? 'Finish' : 'Next'}</button>
                        </div>
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
                    target.style.position = 'relative';
                    target.style.zIndex = '99999';
                    target.style.background = 'white';
                    target.style.pointerEvents = 'none';

                    const rect = target.getBoundingClientRect();
                    if (rect.bottom + 250 < window.innerHeight) {
                        bubble.style.top = (rect.bottom + 16) + 'px';
                    } else {
                        bubble.style.top = (rect.top - bubble.offsetHeight - 16) + 'px';
                    }

                    let leftPos = rect.left;
                    if (leftPos + 320 > window.innerWidth) {
                        leftPos = window.innerWidth - 340;
                    }
                    bubble.style.left = Math.max(16, leftPos) + 'px';
                } else {
                    bubble.style.top = '50%';
                    bubble.style.left = '50%';
                    bubble.style.transform = 'translate(-50%, -50%)';
                }
            }

            function closeWalkthrough() {
                document.querySelectorAll('.walkthrough-highlight, .ohc-walkthrough-highlight').forEach(el => {
                    el.classList.remove('walkthrough-highlight', 'ohc-walkthrough-highlight');
                    el.style.position = '';
                    el.style.zIndex = '';
                    el.style.background = '';
                    el.style.pointerEvents = '';
                });
                if (overlay.parentNode) overlay.parentNode.removeChild(overlay);
                if (bubble.parentNode) bubble.parentNode.removeChild(bubble);
            }

            renderStep();
        };
    }

    // --- Chat Logic ---
    // Inject chat widget HTML if it doesn't exist
    if (!document.getElementById('ohc-help-chat-widget')) {
        const widget = document.createElement('div');
        widget.id = 'ohc-help-chat-widget';
        widget.innerHTML = \`
            <button id="ohc-help-chat-btn" aria-label="Open help chat">?</button>
            <div id="ohc-help-chat-window">
                <div id="ohc-help-chat-header">
                    <h3>Ask AI Help</h3>
                    <button id="ohc-help-chat-close" aria-label="Close help chat">&times;</button>
                </div>
                <div id="ohc-help-chat-messages">
                    <div class="ohc-chat-msg agent">Hello! I am your AI Help Agent. How can I assist you today?</div>
                </div>
                <div id="ohc-help-chat-input-area">
                    <input type="text" id="ohc-help-chat-input" placeholder="Ask anything..." />
                    <button id="ohc-help-chat-send" aria-label="Send message">Send</button>
                </div>
            </div>
        \`;
        document.body.appendChild(widget);

        const chatBtn = document.getElementById('ohc-help-chat-btn');
        const chatWindow = document.getElementById('ohc-help-chat-window');
        const chatClose = document.getElementById('ohc-help-chat-close');
        const chatInput = document.getElementById('ohc-help-chat-input');
        const chatSend = document.getElementById('ohc-help-chat-send');
        const chatMessages = document.getElementById('ohc-help-chat-messages');

        let chatOpen = false;

        chatBtn.addEventListener('click', () => {
            chatOpen = !chatOpen;
            chatWindow.style.display = chatOpen ? 'flex' : 'none';
            if (chatOpen) chatInput.focus();
        });

        chatClose.addEventListener('click', () => {
            chatOpen = false;
            chatWindow.style.display = 'none';
        });

        function appendMessage(text, sender, link = null) {
            const msg = document.createElement('div');
            msg.className = \`ohc-chat-msg \${sender}\`;
            msg.innerHTML = text;
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
                    appendMessage(data.reply.text || data.reply, 'agent', data.reply.link);
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
            if (e.key === 'Enter') handleSend();
        });
    }
});
