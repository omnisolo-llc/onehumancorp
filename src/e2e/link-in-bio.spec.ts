import { test, expect } from './fixtures';
import { createLinkBioActor } from './link_bio_owner';

test.describe('Private Link-in-Bio Configuration', () => {
  test('allows a member to save and reload their own private profile', async ({ page, memberUser, baseURL }) => {
    const memberPage=page;
    const actor=await createLinkBioActor(memberPage,baseURL,memberUser);
    await memberPage.context().grantPermissions(['clipboard-read','clipboard-write'],{origin:new URL(baseURL!).origin});
    await memberPage.goto('/ui/dashboard.html');
    await memberPage.click('#link-in-bio-link');
    await expect(memberPage).toHaveURL(/link-in-bio-generator.html/);
    const save=memberPage.waitForResponse(response=>new URL(response.url()).pathname==='/api/v1/growth/link-in-bio' && response.request().method()==='POST' && response.request().postDataJSON().store_name==='My Awesome Bakery' && response.request().postDataJSON().bio==='The best cookies in town.' && response.request().postDataJSON().theme==='dark');
    await memberPage.fill('#store-name','My Awesome Bakery');
    await memberPage.fill('#bio-text','The best cookies in town.');
    await memberPage.click('.theme-btn[data-theme="dark"]');
    await expect(memberPage.locator('#preview-title')).toHaveText('My Awesome Bakery');
    await expect(memberPage.locator('#preview-bio')).toHaveText('The best cookies in town.');
    const saved=await save;expect(saved.status()).toBe(200);expect(await saved.text()).toBe('');
    expect(saved.request().postDataJSON().tenant_id).toBe(actor.tenantId);
    expect(saved.request().headers()['x-ohc-expected-user']).toBe(actor.userId);
    await expect(memberPage.locator('#private-profile-status')).toContainText('Saved private configuration');
    await memberPage.bringToFront();await expect.poll(()=>memberPage.evaluate(()=>document.hasFocus())).toBe(true);await memberPage.click('#copy-btn');
    await expect(memberPage.locator('#copy-btn')).toHaveText('Copied private preview link');
    expect(await memberPage.evaluate(()=>navigator.clipboard.readText())).toBe(`${new URL(baseURL!).origin}/bio/${encodeURIComponent(actor.tenantId)}`);
    await memberPage.goto(`/ui/bio.html?tenant=${encodeURIComponent(actor.tenantId)}`);
    await expect(memberPage.locator('#title')).toHaveText('My Awesome Bakery');
    await expect(memberPage.locator('#bio')).toHaveText('The best cookies in town.');
    await expect(memberPage.locator('#private-preview-status')).toContainText('Private preview');
    const badge=memberPage.locator('#ohc-badge');await expect(badge).toBeVisible();await expect(badge).toContainText('OmniSolo');
    await memberPage.goto('/ui/link-in-bio-generator.html');
    await expect(memberPage.locator('#store-name')).toHaveValue('My Awesome Bakery');
    const previewBadge=memberPage.locator('#powered-by-link');await expect(previewBadge).toBeVisible();
    await expect(previewBadge).toHaveAttribute('href',/\/api\/v1\/growth\/referrals\/click/);await expect(previewBadge).toHaveAttribute('href',/source=bio_page/);
    const brandingSave=memberPage.waitForResponse(response=>new URL(response.url()).pathname==='/api/v1/growth/link-in-bio' && response.request().method()==='POST' && response.request().postDataJSON().remove_branding===true);
    await memberPage.locator('label',{has:memberPage.locator('#remove-branding-toggle')}).click();
    const brandingResponse=await brandingSave;expect(brandingResponse.status()).toBe(200);expect(await brandingResponse.text()).toBe('');await expect(previewBadge).toBeHidden();
    await memberPage.goto(`/ui/bio.html?tenant=${encodeURIComponent(actor.tenantId)}`);await expect(memberPage.locator('#title')).toHaveText('My Awesome Bakery');await expect(badge).toBeHidden();
  });
});
