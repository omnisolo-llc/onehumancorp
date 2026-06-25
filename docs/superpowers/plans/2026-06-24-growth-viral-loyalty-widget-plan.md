# OHC Viral Loyalty Widget Growth Feature Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enhance the viral loyalty widget to include an interactive mockup and match OHC premium design tokens.

**Architecture:** Modify the static HTML page to add CSS layout improvements and dynamic javascript handling that renders a "card" preview visually updating when "Generate" is clicked. Then add Playwright tests.

**Tech Stack:** HTML/JS, Playwright

## Global Constraints
- **Design Tokens:** Use `rgba(255, 255, 255, 0.65)` and `backdrop-filter: blur(30px) saturate(210%)`.
- **Responsive:** Mobile-first layout target is 375px.

---

### Task 1: Update UI Layout and Styling

**Files:**
- Modify: `src/ui/tauri/src/ui/viral-loyalty-widget.html`

**Interfaces:**
- Consumes: Existing CSS and JS logic.
- Produces: Visual card preview area.

- [ ] **Step 1: Add HTML Structure for Card Preview**
Modify `src/ui/tauri/src/ui/viral-loyalty-widget.html` to add a new `div` for the card preview just above the generate button.

```html
<<<<<<< SEARCH
    <button id="generate-btn">Generate Loyalty Program</button>

    <div id="result-area">
=======
    <div class="card-preview">
       <div class="stamp-grid" id="stamp-grid">
         <div class="stamp empty">1</div>
         <div class="stamp empty">2</div>
         <div class="stamp empty">3</div>
         <div class="stamp empty">4</div>
         <div class="stamp free">FREE</div>
       </div>
    </div>

    <button id="generate-btn">Generate Loyalty Program</button>

    <div id="result-area">
>>>>>>> REPLACE
```

- [ ] **Step 2: Add CSS Styling for Card Preview**
Modify `src/ui/tauri/src/ui/viral-loyalty-widget.html` to include styles for `.card-preview` and `.stamp`.

```html
<<<<<<< SEARCH
    #result-area {
      display: none;
      margin-top: 25px;
      padding-top: 25px;
      border-top: 1px solid rgba(134, 134, 139, 0.2);
    }
  </style>
=======
    #result-area {
      display: none;
      margin-top: 25px;
      padding-top: 25px;
      border-top: 1px solid rgba(134, 134, 139, 0.2);
    }

    .card-preview {
      background: rgba(255, 255, 255, 0.9);
      border-radius: 12px;
      padding: 20px;
      margin-bottom: 25px;
      box-shadow: 0 4px 12px rgba(0,0,0,0.05);
      border: 1px solid rgba(0,0,0,0.05);
    }

    .stamp-grid {
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 8px;
    }

    .stamp {
      width: 48px;
      height: 48px;
      border-radius: 50%;
      border: 2px dashed #ccc;
      display: flex;
      align-items: center;
      justify-content: center;
      font-weight: bold;
      color: #999;
      font-size: 18px;
      transition: all 0.3s;
    }

    .stamp.free {
      background-color: #f0f7ff;
      border-color: #0066FF;
      color: #0066FF;
      font-size: 14px;
    }

    .stamp.filled {
      background-color: #0066FF;
      border-color: #0066FF;
      color: white;
    }

    @media (max-width: 375px) {
      .stamp {
        width: 40px;
        height: 40px;
        font-size: 14px;
      }
      .stamp.free {
        font-size: 11px;
      }
    }
  </style>
>>>>>>> REPLACE
```

- [ ] **Step 3: Add Animation Logic to JS**
Modify `src/ui/tauri/src/ui/viral-loyalty-widget.html` to animate the stamps when generating.

```html
<<<<<<< SEARCH
    document.getElementById('generate-btn').addEventListener('click', async () => {
      const btn = document.getElementById('generate-btn');
      btn.disabled = true;
      btn.textContent = 'Generating...';

      try {
=======
    document.getElementById('generate-btn').addEventListener('click', async () => {
      const btn = document.getElementById('generate-btn');
      btn.disabled = true;
      btn.textContent = 'Generating...';

      const stamps = document.querySelectorAll('.stamp:not(.free)');
      stamps.forEach(s => s.classList.remove('filled'));

      try {
>>>>>>> REPLACE
```

```html
<<<<<<< SEARCH
        // Show result
        const baseUrl = window.location.origin;
        document.getElementById('share-link').value = `${baseUrl}/loyalty/join?ref=${refId}`;
        document.getElementById('result-area').style.display = 'block';
      } catch (err) {
=======
        // Show result
        const baseUrl = window.location.origin;
        document.getElementById('share-link').value = `${baseUrl}/loyalty/join?ref=${refId}`;

        // Animate stamps filling up
        stamps.forEach((stamp, i) => {
            setTimeout(() => {
                stamp.classList.add('filled');
                stamp.textContent = '☕';
            }, i * 200);
        });

        setTimeout(() => {
           document.getElementById('result-area').style.display = 'block';
        }, stamps.length * 200);

      } catch (err) {
>>>>>>> REPLACE
```

### Task 2: Create E2E Test

**Files:**
- Create: `src/e2e/viral-loyalty-widget.spec.ts`

**Interfaces:**
- Consumes: The `viral-loyalty-widget.html` page
- Produces: Passing E2E assertions

- [ ] **Step 1: Write E2E Test File**
Create `src/e2e/viral-loyalty-widget.spec.ts`

```typescript
import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Loyalty Widget', () => {
  test('should load the widget and generate a loyalty program', async ({ page }) => {
    // We mock the backend response here specifically because this is a static UI page
    // in the tauri bundle that simulates growth mechanics.
    await page.route('/api/v1/growth/referrals/generate', async route => {
      await route.fulfill({ json: { referral_link: 'http://example.com/ref/12345' } });
    });

    await page.goto('/ui/viral-loyalty-widget.html');

    // Wait for main elements
    await expect(page.locator('h1')).toHaveText('Viral Loyalty Widget Generator');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // Check initial stamps state
    const emptyStamps = page.locator('.stamp.empty');
    await expect(emptyStamps).toHaveCount(4);

    // Click generate
    await generateBtn.click();

    // Verify animation starts
    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    // Wait for the animation to finish and result to show
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Verify filled stamps
    const filledStamps = page.locator('.stamp.filled');
    await expect(filledStamps).toHaveCount(4);

    // Check share link generated correctly
    const shareLink = page.locator('#share-link');
    await expect(shareLink).toHaveValue(/loyalty\/join\?ref=12345/);
  });
});
```
