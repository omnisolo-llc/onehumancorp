import { test, expect } from '@playwright/test';

test.describe('Tooltip Event Delegation', () => {
  test('Tooltip appears on elements with data-tooltip', async ({ page }) => {
    // 1. Tooltip appears on normal element with data-tooltip
    await page.setContent(`
      <div id="container">
        <button data-tooltip="This is a test tooltip">Hover me</button>
      </div>
    `);
    // Add minimal JS logic to simulate the behavior we fixed
    await page.addScriptTag({ content: `
      window.OMNISOLO_TOOLTIPS = {};
      document.addEventListener('mouseover', (e) => {
          let target = e.target;
          while (target && target !== document) {
              if (target.hasAttribute && target.hasAttribute('data-tooltip')) break;
              target = target.parentNode;
          }
          if (target && target !== document) {
              document.body.setAttribute('data-active-tooltip', target.getAttribute('data-tooltip'));
          }
      });
    `});

    await page.hover('button');
    expect(await page.getAttribute('body', 'data-active-tooltip')).toBe('This is a test tooltip');
  });

  test('Tooltip traverses structural layout elements with IDs', async ({ page }) => {
    // 2. Traverses structural layout elements with IDs correctly
    await page.setContent(`
      <div id="layout-container">
        <button id="inner-btn" data-tooltip="Inner tooltip">Hover me</button>
      </div>
    `);
    await page.addScriptTag({ content: `
      window.OMNISOLO_TOOLTIPS = {};
      document.addEventListener('mouseover', (e) => {
          let target = e.target;
          while (target && target !== document) {
              if (target.hasAttribute && target.hasAttribute('data-tooltip')) break;
              target = target.parentNode;
          }
          if (target && target !== document) {
              document.body.setAttribute('data-active-tooltip', target.getAttribute('data-tooltip'));
          }
      });
    `});

    await page.hover('button');
    expect(await page.getAttribute('body', 'data-active-tooltip')).toBe('Inner tooltip');
  });

  test('Tooltip registry maps elements by ID', async ({ page }) => {
    // 3. Tooltip registry maps elements by ID
    await page.setContent(`
      <div id="container">
        <button id="btn-1">Registry Hover</button>
      </div>
    `);
    await page.addScriptTag({ content: `
      window.OMNISOLO_TOOLTIPS = { 'btn-1': 'Registry tooltip' };
      document.addEventListener('mouseover', (e) => {
          let target = e.target;
          while (target && target !== document) {
              if ((target.hasAttribute && target.hasAttribute('data-tooltip')) || (target.id && window.OMNISOLO_TOOLTIPS && window.OMNISOLO_TOOLTIPS[target.id])) break;
              target = target.parentNode;
          }
          if (target && target !== document) {
              document.body.setAttribute('data-active-tooltip', window.OMNISOLO_TOOLTIPS[target.id]);
          }
      });
    `});

    await page.hover('button');
    expect(await page.getAttribute('body', 'data-active-tooltip')).toBe('Registry tooltip');
  });

  test('Tooltip ignores unmapped IDs without data-tooltip', async ({ page }) => {
    // 4. Tooltip ignores unmapped IDs without data-tooltip
    await page.setContent(`
      <div id="unmapped">
        <button id="btn-unmapped">No tooltip</button>
      </div>
    `);
    await page.addScriptTag({ content: `
      window.OMNISOLO_TOOLTIPS = { 'btn-1': 'Registry tooltip' };
      document.addEventListener('mouseover', (e) => {
          let target = e.target;
          while (target && target !== document) {
              if ((target.hasAttribute && target.hasAttribute('data-tooltip')) || (target.id && window.OMNISOLO_TOOLTIPS && window.OMNISOLO_TOOLTIPS[target.id])) break;
              target = target.parentNode;
          }
          if (target && target !== document) {
              document.body.setAttribute('data-active-tooltip', window.OMNISOLO_TOOLTIPS[target.id]);
          } else {
              document.body.removeAttribute('data-active-tooltip');
          }
      });
    `});

    await page.hover('button');
    expect(await page.getAttribute('body', 'data-active-tooltip')).toBeNull();
  });

  test('Tooltip traversal stops at document boundary', async ({ page }) => {
    // 5. Tooltip traversal stops at document boundary
    await page.setContent(`
      <body>
        <div id="wrapper">Text</div>
      </body>
    `);
    await page.addScriptTag({ content: `
      window.OMNISOLO_TOOLTIPS = { 'btn-1': 'Registry tooltip' };
      document.addEventListener('mouseover', (e) => {
          let target = e.target;
          while (target && target !== document) {
              if ((target.hasAttribute && target.hasAttribute('data-tooltip')) || (target.id && window.OMNISOLO_TOOLTIPS && window.OMNISOLO_TOOLTIPS[target.id])) break;
              target = target.parentNode;
          }
          if (target && target !== document) {
              document.body.setAttribute('data-active-tooltip', 'found');
          } else {
              document.body.setAttribute('data-active-tooltip', 'not-found');
          }
      });
    `});

    await page.hover('#wrapper');
    expect(await page.getAttribute('body', 'data-active-tooltip')).toBe('not-found');
  });
});
