document.addEventListener('DOMContentLoaded', () => {
    // Tooltips
    let tooltipEl = document.querySelector('.ohc-tooltip');
    if (!tooltipEl) {
        tooltipEl = document.createElement('div');
        tooltipEl.className = 'ohc-tooltip';
        document.body.appendChild(tooltipEl);
    }

    if (!window.OHC_TOOLTIPS) {
        fetch("/api/tooltips").then(r => r.json()).then(data => { window.OHC_TOOLTIPS = data; }).catch(e => {
            console.error("Failed to fetch tooltips", e);
        });
    }

    function showTooltip(e, text) {
        if (!text || !tooltipEl) return;
        tooltipEl.textContent = text;

        let targetRect = null;
        if (e.target && e.target.getBoundingClientRect) {
            targetRect = e.target.getBoundingClientRect();
        } else if (e.touches && e.touches.length > 0) {
            targetRect = { left: e.touches[0].clientX, width: 0, top: e.touches[0].clientY, height: 0 };
        } else if (e.clientX !== undefined) {
             targetRect = { left: e.clientX, width: 0, top: e.clientY, height: 0 };
        }

        if (targetRect) {
            let top = targetRect.top - tooltipEl.offsetHeight - 10;
            let left = targetRect.left + (targetRect.width / 2) - (tooltipEl.offsetWidth / 2);

            if (left < 10) left = 10;
            if (left + tooltipEl.offsetWidth > window.innerWidth - 10) {
                left = window.innerWidth - tooltipEl.offsetWidth - 10;
            }
            if (top < 10) {
                top = targetRect.top + targetRect.height + 10;
            }
            if (top + tooltipEl.offsetHeight > window.innerHeight - 10) {
                top = targetRect.top - tooltipEl.offsetHeight - 10;
            }

            tooltipEl.style.top = top + 'px';
            tooltipEl.style.left = left + 'px';
            tooltipEl.classList.add('visible');
        }
    }

    function hideTooltip() {
        if (tooltipEl) {
            tooltipEl.classList.remove('visible');
        }
    }

    document.addEventListener('mouseover', (e) => {
        const target = e.target.closest('[data-tooltip], [id]');
        if (target) {
            const text = (window.OHC_TOOLTIPS && target.id && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip');
            if (text) showTooltip(e, text);
        }
    });

    document.addEventListener('mouseout', (e) => {
        hideTooltip();
    });

    document.addEventListener('touchstart', (e) => {
        const target = e.target.closest('[data-tooltip], [id]');
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

    // Walkthroughs
    if (!window.startWalkthrough) {
        let currentSteps = [];
        let currentStepIndex = 0;
        let overlayEl = null;
        let bubbleEl = null;
        let highlightEl = null;

        function endWalkthrough() {
            if (overlayEl) {
                overlayEl.remove();
                overlayEl = null;
            }
            if (highlightEl) {
                highlightEl.remove();
                highlightEl = null;
            }
            if (bubbleEl) {
                bubbleEl.remove();
                bubbleEl = null;
            }
            currentSteps = [];
            currentStepIndex = 0;
            document.querySelectorAll('.walkthrough-highlight, .ohc-walkthrough-highlight').forEach(el => {
                el.classList.remove('walkthrough-highlight', 'ohc-walkthrough-highlight');
            });
        }

        window.startWalkthrough = function(steps) {
            if (!steps || steps.length === 0) return;
            currentSteps = steps;
            currentStepIndex = 0;

            endWalkthrough();

            overlayEl = document.createElement('div');
            overlayEl.id = 'walkthrough-overlay';
            overlayEl.className = 'ohc-walkthrough-overlay';

            highlightEl = document.createElement('div');
            highlightEl.className = 'ohc-walkthrough-highlight';

            bubbleEl = document.createElement('div');
            bubbleEl.id = 'walkthrough-bubble';
            bubbleEl.className = 'ohc-walkthrough-bubble';
            bubbleEl.setAttribute('role', 'dialog');

            document.body.appendChild(overlayEl);
            document.body.appendChild(highlightEl);
            document.body.appendChild(bubbleEl);

            renderStep();
        };

        function renderStep() {
            const step = currentSteps[currentStepIndex];
            if (!step) {
                endWalkthrough();
                return;
            }

            if (bubbleEl) {
                bubbleEl.setAttribute("aria-label", (step.title || "Tour") + " walkthrough step");
            }

            document.querySelectorAll('.walkthrough-highlight, .ohc-walkthrough-highlight').forEach(el => {
                el.classList.remove('walkthrough-highlight', 'ohc-walkthrough-highlight');
            });

            // Handle different step formats (targetId vs selector)
            let targetEl = null;
            if (step.selector) {
                targetEl = document.querySelector(step.selector);
            } else if (step.targetId) {
                targetEl = document.querySelector(step.targetId) || document.getElementById(step.targetId.replace('#', ''));
            }

            if (targetEl) {
                targetEl.classList.add('ohc-walkthrough-highlight');
                targetEl.scrollIntoView({ behavior: 'smooth', block: 'center' });
                const rect = targetEl.getBoundingClientRect();

                if (highlightEl) {
                    highlightEl.style.top = (rect.top - 4) + 'px';
                    highlightEl.style.left = (rect.left - 4) + 'px';
                    highlightEl.style.width = (rect.width + 8) + 'px';
                    highlightEl.style.height = (rect.height + 8) + 'px';
                }

                if (bubbleEl) {
                    let bubbleTop = rect.bottom + 16;
                    if (bubbleTop + 200 > window.innerHeight) {
                        bubbleTop = Math.max(16, rect.top - 200);
                    }
                    bubbleEl.style.top = bubbleTop + 'px';
                    bubbleEl.style.left = Math.max(16, Math.min(rect.left, window.innerWidth - 320)) + 'px';
                }
            } else {
                if (highlightEl) {
                    highlightEl.style.display = 'none';
                }
                if (bubbleEl) {
                    bubbleEl.style.top = '50%';
                    bubbleEl.style.left = '50%';
                    bubbleEl.style.transform = 'translate(-50%, -50%)';
                }
            }

            if (bubbleEl) {
                bubbleEl.innerHTML = `
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                        <h4 style="margin: 0; font-size: 16px; color: #1d1d1f;">${step.title}</h4>
                        <button class="ohc-walkthrough-close" style="background: none; border: none; font-size: 20px; cursor: pointer; color: #86868b; padding: 0;">&times;</button>
                    </div>
                    <p style="margin: 0 0 16px 0; font-size: 14px; color: #1d1d1f; line-height: 1.4;">${step.content || step.text || ''}</p>
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <span style="font-size: 12px; color: #86868b;">Step ${currentStepIndex + 1} of ${currentSteps.length}</span>
                        <div>
                            ${currentStepIndex > 0 ? '<button class="ohc-walkthrough-prev" style="background: none; border: none; color: #0066FF; cursor: pointer; font-size: 14px; font-weight: 500; margin-right: 12px;">Back</button>' : ''}
                            <button class="ohc-walkthrough-next" style="background: #0066FF; border: none; color: white; padding: 6px 12px; border-radius: 6px; cursor: pointer; font-size: 14px; font-weight: 500;">${currentStepIndex < currentSteps.length - 1 ? 'Next' : 'Finish'}</button>
                        </div>
                    </div>
                `;

                bubbleEl.querySelector('.ohc-walkthrough-close').addEventListener('click', endWalkthrough);
                if (currentStepIndex > 0) {
                    bubbleEl.querySelector('.ohc-walkthrough-prev').addEventListener('click', () => {
                        currentStepIndex--;
                        renderStep();
                    });
                }
                bubbleEl.querySelector('.ohc-walkthrough-next').addEventListener('click', () => {
                    if (currentStepIndex < currentSteps.length - 1) {
                        currentStepIndex++;
                        renderStep();
                    } else {
                        endWalkthrough();
                    }
                });
            }
        }
    }
});
