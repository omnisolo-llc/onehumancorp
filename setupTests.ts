import '@testing-library/jest-dom/vitest';

if (typeof window !== 'undefined' && typeof window.HTMLElement !== 'undefined') {
    window.HTMLElement.prototype.scrollIntoView = function () {};
}
