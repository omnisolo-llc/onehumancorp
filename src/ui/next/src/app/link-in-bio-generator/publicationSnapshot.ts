type BioPublicationFields = {
  storeName: string;
  bio: string;
  links: ReadonlyArray<{ title: string; url: string }>;
};

/** Select public content only; the shared publication client owns review and admission. */
export function bioPublicationSnapshot(profile: BioPublicationFields) {
  return {
    domain: null,
    pages: [{
      path: '/',
      title: profile.storeName,
      seo_metadata: { name: profile.storeName, description: profile.bio },
      blocks: [
        { block_type: 'HeroBlock', content: { headline: profile.storeName, subtitle: profile.bio }, sort_order: 0 },
        { block_type: 'LinkListBlock', content: { links: profile.links.map(link => ({ label: link.title, url: link.url })) }, sort_order: 1 },
      ],
    }],
  };
}
