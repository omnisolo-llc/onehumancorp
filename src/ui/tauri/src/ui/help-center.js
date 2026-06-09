/**
 * OHC Contextual Tooltips Registry
 */
class TooltipRegistry {
    constructor() {
        this.tooltips = new Map();
        this.tooltipEl = document.createElement('div');
        this.tooltipEl.className = 'ohc-tooltip';
        this.tooltipEl.id = 'ohc-global-tooltip';
        document.body.appendChild(this.tooltipEl);

        this.activeElement = null;
        this.showTimeout = null;

        // Mobile detection
        this.isTouch = ('ontouchstart' in window) || (navigator.maxTouchPoints > 0);
    }

    register(selector, text) {
        this.tooltips.set(selector, text);
        this._attachListeners(selector);
    }

    _attachListeners(selector) {
        const elements = document.querySelectorAll(selector);
        elements.forEach(el => {
            // Avoid duplicate listeners
            if (el.dataset.hasTooltip) return;
            el.dataset.hasTooltip = "true";

            if (this.isTouch) {
                // Long press for mobile
                let pressTimer;
                el.addEventListener('touchstart', (e) => {
                    pressTimer = window.setTimeout(() => {
                        this.show(el, selector);
                    }, 500);
                });
                el.addEventListener('touchend', () => {
                    clearTimeout(pressTimer);
                    setTimeout(() => this.hide(), 2000); // Hide after a delay
                });
                el.addEventListener('touchmove', () => {
                    clearTimeout(pressTimer);
                });
            } else {
                // Hover for desktop
                el.addEventListener('mouseenter', () => {
                    this.showTimeout = setTimeout(() => this.show(el, selector), 300);
                });
                el.addEventListener('mouseleave', () => {
                    clearTimeout(this.showTimeout);
                    this.hide();
                });
                el.addEventListener('focus', () => this.show(el, selector));
                el.addEventListener('blur', () => this.hide());
            }
        });
    }

    show(element, selector) {
        const text = this.tooltips.get(selector);
        if (!text) return;

        this.tooltipEl.textContent = text;
        const rect = element.getBoundingClientRect();

        // Calculate position (centered above element by default)
        const tooltipRect = this.tooltipEl.getBoundingClientRect();
        let top = rect.top - tooltipRect.height - 10;
        let left = rect.left + (rect.width / 2) - (tooltipRect.width / 2);

        // Adjust if off-screen
        if (top < 10) {
            top = rect.bottom + 10; // place below
        }
        if (left < 10) {
            left = 10;
        } else if (left + tooltipRect.width > window.innerWidth - 10) {
            left = window.innerWidth - tooltipRect.width - 10;
        }

        this.tooltipEl.style.top = `${top + window.scrollY}px`;
        this.tooltipEl.style.left = `${left + window.scrollX}px`;
        this.tooltipEl.classList.add('visible');
    }

    hide() {
        this.tooltipEl.classList.remove('visible');
    }

    // Refresh listeners for dynamically added elements
    refresh() {
        this.tooltips.forEach((text, selector) => {
            this._attachListeners(selector);
        });
    }
}

/**
 * OHC In-App Help Center
 */
class HelpCenter {
    constructor() {
        this.articles = [];

        this.render();
        this.attachEvents();
        this.loadArticles();
    }

    loadArticles() {
        if (window.__TAURI__ && window.__TAURI__.core) {
            window.__TAURI__.core.invoke("get_help_articles").then((articles) => {
                if (articles && articles.length > 0) {
                    this.articles = articles;
                    this.renderArticles(this.articles);
                } else {
                    this.contentEl.innerHTML = '<p style="color:#86868b; text-align:center; margin-top:40px;">No articles available yet.</p>';
                }
            }).catch(e => {
                console.error(e);
                this.contentEl.innerHTML = '<p style="color:#86868b; text-align:center; margin-top:40px;">Failed to load articles.</p>';
            });
        } else {
            // E2E test environment without Tauri backend
            fetch('/api/help-articles').then(r => r.json()).then(articles => {
                if (articles && articles.length > 0) {
                    this.articles = articles;
                    this.renderArticles(this.articles);
                } else {
                    this.contentEl.innerHTML = '<p style="color:#86868b; text-align:center; margin-top:40px;">No articles available yet.</p>';
                }
            }).catch(e => {
                this.contentEl.innerHTML = '<p style="color:#86868b; text-align:center; margin-top:40px;">Failed to load articles.</p>';
            });
        }
    }

    render() {
        // Create FAB
        this.fab = document.createElement('div');
        this.fab.className = 'ohc-help-fab';
        this.fab.id = 'ohc-help-fab';
        this.fab.innerHTML = '?';
        document.body.appendChild(this.fab);

        // Create Sidebar Overlay
        this.sidebar = document.createElement('div');
        this.sidebar.className = 'ohc-help-sidebar';
        this.sidebar.id = 'ohc-help-sidebar';

        this.sidebar.innerHTML = `
            <div class="ohc-help-header">
                <h2>Help Center</h2>
                <button class="ohc-help-close" id="ohc-help-close">&times;</button>
            </div>

            <div class="ohc-help-search-container">
                <input type="text" class="ohc-help-search" id="ohc-help-search" placeholder="Search for help...">
            </div>

            <div class="ohc-help-content" id="ohc-help-content">
                <!-- Articles injected here -->
            </div>

            <div class="ohc-help-footer">
                <button class="ohc-help-ask-btn" id="ohc-help-ask-btn">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path></svg>
                    Ask anything
                </button>
            </div>

            <!-- Article View Sub-panel -->
            <div class="ohc-article-view" id="ohc-article-view">
                <div class="ohc-help-header">
                    <button class="ohc-back-btn" id="ohc-article-back">← Back</button>
                </div>
                <div class="ohc-article-body" id="ohc-article-body"></div>
            </div>

            <!-- Chat View Sub-panel -->
            <div class="ohc-chat-view" id="ohc-chat-view">
                <div class="ohc-help-header">
                    <button class="ohc-back-btn" id="ohc-chat-back">← Back</button>
                    <h2 style="font-size:16px;">Help Agent</h2>
                </div>
                <div class="ohc-chat-messages" id="ohc-chat-messages">
                    <div class="ohc-chat-bubble bot">Hi! I'm your OHC Help Agent. How can I assist you today?</div>
                </div>
                <div class="ohc-chat-input-container">
                    <input type="text" class="ohc-chat-input" id="ohc-chat-input" placeholder="Type your question...">
                    <button class="ohc-chat-send" id="ohc-chat-send">➤</button>
                </div>
            </div>
        `;
        document.body.appendChild(this.sidebar);

        this.contentEl = document.getElementById('ohc-help-content');
        this.renderArticles(this.articles);
    }

    renderArticles(articles) {
        this.contentEl.innerHTML = '';

        if (articles.length === 0) {
            this.contentEl.innerHTML = '<p style="color:#86868b; text-align:center; margin-top:40px;">No articles found.</p>';
            return;
        }

        // Group by category
        const groups = {};
        articles.forEach(a => {
            if (!groups[a.category]) groups[a.category] = [];
            groups[a.category].push(a);
        });

        for (const [category, items] of Object.entries(groups)) {
            const catEl = document.createElement('div');
            catEl.className = 'ohc-help-category';
            catEl.innerHTML = `
                <div class="ohc-help-category-title">${category}</div>
                <ul class="ohc-help-article-list">
                    ${items.map(item => `
                        <li class="ohc-help-article-item">
                            <a href="#" class="ohc-help-article-link" data-id="${item.id}">${item.title}</a>
                        </li>
                    `).join('')}
                </ul>
            `;
            this.contentEl.appendChild(catEl);
        }

        // Attach article click events
        const links = this.contentEl.querySelectorAll('.ohc-help-article-link');
        links.forEach(link => {
            link.addEventListener('click', (e) => {
                e.preventDefault();
                const id = link.getAttribute('data-id');
                this.openArticle(id);
            });
        });
    }

    openArticle(id) {
        const article = this.articles.find(a => a.id === id);
        if (article) {
            document.getElementById('ohc-article-body').innerHTML = article.content;
            document.getElementById('ohc-article-view').classList.add('open');
        }
    }

    closeArticle() {
        document.getElementById('ohc-article-view').classList.remove('open');
    }

    openChat() {
        document.getElementById('ohc-chat-view').classList.add('open');
    }

    closeChat() {
        document.getElementById('ohc-chat-view').classList.remove('open');
    }

    handleChatSend() {
        const input = document.getElementById('ohc-chat-input');
        const text = input.value.trim();
        if (!text) return;

        const messagesEl = document.getElementById('ohc-chat-messages');

        // Add User Message
        const userMsg = document.createElement('div');
        userMsg.className = 'ohc-chat-bubble user';
        userMsg.textContent = text;
        messagesEl.appendChild(userMsg);

        input.value = '';
        messagesEl.scrollTop = messagesEl.scrollHeight;

        if (window.__TAURI__ && window.__TAURI__.core) {
            window.__TAURI__.core.invoke("ask_help_agent", { query: text }).then((response) => {
                const botMsg = document.createElement('div');
                botMsg.className = 'ohc-chat-bubble bot';
                botMsg.innerHTML = response;
                messagesEl.appendChild(botMsg);
                messagesEl.scrollTop = messagesEl.scrollHeight;
            }).catch(e => {
                const botMsg = document.createElement('div');
                botMsg.className = 'ohc-chat-bubble bot';
                botMsg.innerHTML = "Sorry, I'm having trouble connecting right now.";
                messagesEl.appendChild(botMsg);
                messagesEl.scrollTop = messagesEl.scrollHeight;
            });
        } else {
            // E2E test environment fallback
            fetch('/api/ask-help-agent', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ query: text })
            }).then(r => r.text()).then(response => {
                const botMsg = document.createElement('div');
                botMsg.className = 'ohc-chat-bubble bot';
                botMsg.innerHTML = response;
                messagesEl.appendChild(botMsg);
                messagesEl.scrollTop = messagesEl.scrollHeight;
            }).catch(e => {
                const botMsg = document.createElement('div');
                botMsg.className = 'ohc-chat-bubble bot';
                botMsg.innerHTML = "Sorry, I'm having trouble connecting right now.";
                messagesEl.appendChild(botMsg);
                messagesEl.scrollTop = messagesEl.scrollHeight;
            });
        }
    }

    attachEvents() {
        this.fab.addEventListener('click', () => {
            this.sidebar.classList.add('open');
        });

        document.getElementById('ohc-help-close').addEventListener('click', () => {
            this.sidebar.classList.remove('open');
            this.closeArticle();
            this.closeChat();
        });

        document.getElementById('ohc-article-back').addEventListener('click', () => {
            this.closeArticle();
        });

        document.getElementById('ohc-chat-back').addEventListener('click', () => {
            this.closeChat();
        });

        document.getElementById('ohc-help-ask-btn').addEventListener('click', () => {
            this.openChat();
        });

        document.getElementById('ohc-chat-send').addEventListener('click', () => {
            this.handleChatSend();
        });

        document.getElementById('ohc-chat-input').addEventListener('keydown', (e) => {
            if (e.key === 'Enter') this.handleChatSend();
        });

        // Search functionality
        const searchInput = document.getElementById('ohc-help-search');
        searchInput.addEventListener('input', (e) => {
            const query = e.target.value.toLowerCase();
            const filtered = this.articles.filter(a =>
                a.title.toLowerCase().includes(query) ||
                a.content.toLowerCase().includes(query)
            );
            this.renderArticles(filtered);
        });
    }
}

// Initialization on DOM Content Loaded
document.addEventListener('DOMContentLoaded', () => {
    window.ohcTooltipRegistry = new TooltipRegistry();
    window.ohcHelpCenter = new HelpCenter();

    // Register common tooltips across the app
    window.ohcTooltipRegistry.register('.status-badge', 'Indicates your current connectivity status.');
    window.ohcTooltipRegistry.register('#dashboard-btn', 'Go to your main workspace overview.');
    window.ohcTooltipRegistry.register('#generate-link-btn', 'Generates a secure invite link for a new team member.');
    window.ohcTooltipRegistry.register('#start-btn', 'Begin the onboarding process.');
});
