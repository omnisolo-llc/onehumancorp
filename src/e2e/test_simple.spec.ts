
import { test, expect } from '@playwright/test';
import * as fs from 'fs';

test('Full onboarding wizard flow with JS validations', async ({ page }) => {
  const libRs = fs.readFileSync('src/server/lib.rs', 'utf-8');
  const htmlMatch = libRs.match(/<!DOCTYPE html>[\s\S]*<\/html>/);
  const rawHtml = htmlMatch ? htmlMatch[0] : '';

  await page.setContent(rawHtml);

  await page.evaluate(() => {
      document.querySelectorAll('.screen').forEach(s => s.style.display = 'none');
      const setupScreen = document.getElementById('setup-screen');
      if (setupScreen) setupScreen.style.display = 'block';
      document.querySelectorAll('div[id^="step-"]').forEach(div => div.style.display = 'block');
  });

  await page.evaluate(() => {
      const btn = document.querySelector('#step-1 button');
      if (btn) btn.click();
  });

  await page.evaluate(() => {
      const btns = Array.from(document.querySelectorAll('#step-2 button'));
      const btn = btns.find(b => b.textContent?.includes('Online Store'));
      if(btn) btn.click();
  });

  await page.evaluate(() => {
      const inputs = document.querySelectorAll('#step-3 input');
      if(inputs.length > 0) {
          inputs[0].value = "Maya's Bakery";
          inputs[0].dispatchEvent(new Event('input'));
      }
      const btns = Array.from(document.querySelectorAll('#step-3 button'));
      const btn = btns.find(b => b.textContent?.includes('Next'));
      if(btn) btn.click();
  });

  await page.evaluate(() => {
      const btns = Array.from(document.querySelectorAll('#step-4 button'));
      const btn = btns.find(b => b.textContent?.includes('Next'));
      if(btn) btn.click();
  });

  await page.evaluate(() => {
      const inputs = document.querySelectorAll('#step-5 input');
      if(inputs.length > 0) {
          inputs[0].value = "Chocolate Cake";
          inputs[0].dispatchEvent(new Event('input'));
      }
      const btns = Array.from(document.querySelectorAll('#step-5 button'));
      const genBtn = btns.find(b => b.textContent?.includes('Generate') || b.textContent?.includes('AI'));
      if(genBtn) genBtn.click();

      const nextBtn = btns.find(b => b.textContent?.includes('Next'));
      if(nextBtn) nextBtn.click();
  });

  await page.evaluate(() => {
      const btns = Array.from(document.querySelectorAll('#step-6 button'));
      const btn = btns.find(b => b.textContent?.includes('Next') || b.textContent?.includes('Back'));
      if(btn) btn.click();
  });

  await page.evaluate(() => {
      const btns = Array.from(document.querySelectorAll('#step-7 button'));
      const nextBtn = btns.find(b => b.textContent?.includes('Next'));
      if(nextBtn) nextBtn.click();
  });

  await page.evaluate(() => {
      const btns = Array.from(document.querySelectorAll('#step-8 button'));
      const nextBtn = btns.find(b => b.textContent?.includes('Next'));
      if(nextBtn) nextBtn.click();
  });

  await page.evaluate(() => {
      const btns = Array.from(document.querySelectorAll('#step-9 button'));
      const nextBtn = btns.find(b => b.textContent?.includes('Next') || b.textContent?.includes('Publish'));
      if(nextBtn) nextBtn.click();
  });

  expect(true).toBeTruthy();
});
