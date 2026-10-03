import { test, expect } from './fixtures';

test('unconfigured dashboard memory retains an unsent draft without fabricated recall', async ({ page }) => {
  const writes:string[]=[];
  page.on('request',request=>{
    if(request.method()!=='GET' && /\/(?:assistant\/)?memory(?:\/|$)/.test(new URL(request.url()).pathname)) writes.push(request.url());
  });
  await page.goto('/');
  await expect(page.getByText('Unified Agent Feed',{exact:true})).toBeVisible();
  // Legacy browser content has no verified owner and cannot become memory.
  await page.evaluate(()=>localStorage.setItem('user_favorite_cake','Unowned legacy preference'));
  const input=page.getByPlaceholder('Message...', {exact:true});
  await input.fill('My favorite cake is chocolate');
  await page.getByRole('button',{name:'Send',exact:true}).click();
  await expect(page.getByRole('alert').filter({hasText:'Your draft has not been sent or saved'})).toBeVisible();
  await expect(input).toHaveValue('My favorite cake is chocolate');
  await expect(page.locator('.agent-message')).toHaveCount(0);

  await page.reload();
  await input.fill('What is my favorite cake?');
  await page.getByRole('button',{name:'Send',exact:true}).click();
  await expect(page.getByRole('alert').filter({hasText:'Your draft has not been sent or saved'})).toBeVisible();
  await expect(page.locator('.agent-message')).toHaveCount(0);
  await expect(page.getByText('Unowned legacy preference',{exact:true})).toHaveCount(0);
  expect(await page.evaluate(()=>localStorage.getItem('user_favorite_cake'))).toBe('Unowned legacy preference');
  expect(writes).toEqual([]);
  // This is negative safety coverage. Durable actor-bound memory and any model
  // consolidation/recall remain required open implementation/acceptance work.
});
