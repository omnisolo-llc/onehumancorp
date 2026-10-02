import { describe, expect, it } from 'vitest';
import { bioPublicationSnapshot } from './publicationSnapshot';

describe('the public bio snapshot boundary', () => {
  it('copies only the explicitly selected public profile fields', () => {
    const profile = {
      storeName: 'Owner Bakery', bio: 'The reviewed description',
      links: [{ id: 'private-row', title: 'Our menu', url: 'https://example.test/menu' }],
      tenantId: 'private-tenant', userId: 'private-user', theme: 'dark',
      removeBranding: true, domain: 'unverified.example.test', internalNote: 'Private note',
    };
    expect(bioPublicationSnapshot(profile)).toEqual({
      domain: null,
      pages: [{
        path: '/', title: 'Owner Bakery',
        seo_metadata: { name: 'Owner Bakery', description: 'The reviewed description' },
        blocks: [
          { block_type: 'HeroBlock', content: { headline: 'Owner Bakery', subtitle: 'The reviewed description' }, sort_order: 0 },
          { block_type: 'LinkListBlock', content: { links: [{ label: 'Our menu', url: 'https://example.test/menu' }] }, sort_order: 1 },
        ],
      }],
    });
  });

  it('preserves the owner-entered text and destination without constructing a public address', () => {
    const snapshot = bioPublicationSnapshot({
      storeName: '雪 & "Tea"', bio: '<literal text>\nSecond line',
      links: [{ title: "Owner's menu", url: 'https://example.test/雪?x=1&label=%22tea%22' }],
    });
    expect(snapshot.pages[0].blocks[0].content).toEqual({ headline: '雪 & "Tea"', subtitle: '<literal text>\nSecond line' });
    expect(snapshot.pages[0].blocks[1].content).toEqual({ links: [{ label: "Owner's menu", url: 'https://example.test/雪?x=1&label=%22tea%22' }] });
    expect(snapshot).not.toHaveProperty('public_path');
    expect(snapshot).not.toHaveProperty('site_id');
  });

  it('takes a fresh copy so later draft edits cannot mutate the earlier review', () => {
    const profile = { storeName: 'First name', bio: 'First bio', links: [{ title: 'First link', url: 'https://example.test/first' }] };
    const first = bioPublicationSnapshot(profile);
    profile.storeName = 'Later name'; profile.bio = 'Later bio';
    profile.links[0].title = 'Later link'; profile.links[0].url = 'https://example.test/later';
    profile.links.push({ title: 'New link', url: 'https://example.test/new' });
    expect(first.pages[0].title).toBe('First name');
    expect(first.pages[0].blocks[0].content).toEqual({ headline: 'First name', subtitle: 'First bio' });
    expect(first.pages[0].blocks[1].content).toEqual({ links: [{ label: 'First link', url: 'https://example.test/first' }] });
    expect(bioPublicationSnapshot(profile)).not.toEqual(first);
  });

  it('preserves configured link order and keeps an empty list empty', () => {
    const profile = { storeName: 'Profile', bio: '', links: [
      { title: 'Second alphabetically', url: 'http://example.test/two' },
      { title: 'First alphabetically', url: 'https://example.test/one' },
    ] };
    expect(bioPublicationSnapshot(profile).pages[0].blocks[1].content).toEqual({ links: [
      { label: 'Second alphabetically', url: 'http://example.test/two' },
      { label: 'First alphabetically', url: 'https://example.test/one' },
    ] });
    expect(bioPublicationSnapshot({ ...profile, links: [] }).pages[0].blocks[1].content).toEqual({ links: [] });
  });
});
