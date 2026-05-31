(function() {
    // Wall of Love Widget
    const container = document.getElementById('ohc-wall-of-love');
    if (!container) return;

    const storeName = container.getAttribute('data-store') || 'This Store';

    // Mocked reviews for the "smoke and mirrors" prototype
    const reviews = [
        { name: "Sarah J.", rating: 5, text: `Absolutely love the products from ${storeName}! Will definitely buy again.` },
        { name: "Michael T.", rating: 5, text: "Fast shipping and amazing quality. Highly recommended!" },
        { name: "Emily R.", rating: 5, text: "Customer service was top notch, and the item exceeded my expectations." },
    ];

    let html = `
        <div style="font-family: system-ui, -apple-system, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px; border-radius: 12px; background: linear-gradient(to right, #ffffff, #fcfbf8); box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06); border: 1px solid rgba(167, 139, 250, 0.2);">
            <h3 style="text-align: center; color: #1f2937; margin-bottom: 20px; font-weight: 700; font-size: 1.25rem;">Loved by customers</h3>
            <div style="display: flex; flex-direction: column; gap: 16px;">
    `;

    reviews.forEach(review => {
        html += `
                <div style="background: white; padding: 16px; border-radius: 8px; border: 1px solid #f3f4f6;">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                        <span style="font-weight: 600; color: #374151;">${review.name}</span>
                        <span style="color: #fbbf24;">${'★'.repeat(review.rating)}</span>
                    </div>
                    <p style="color: #6b7280; font-size: 0.875rem; line-height: 1.5; margin: 0;">"${review.text}"</p>
                </div>
        `;
    });

    html += `
            </div>
            <div style="text-align: center; margin-top: 16px; font-size: 0.75rem; color: #9ca3af;">
                <span style="display: flex; align-items: center; justify-content: center; gap: 4px;">
                    ⚡ Powered by <a href="https://ohc.store/join?ref=${encodeURIComponent(storeName)}" style="color: #8b5cf6; text-decoration: none; font-weight: 600;" target="_blank">OHC</a>
                </span>
            </div>
        </div>
    `;

    container.innerHTML = html;
})();
