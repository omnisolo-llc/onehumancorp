import React, { useState } from 'react';

// HelpCenter Component for non-technical small business owners.
// Focuses on plain language and clear, mobile-friendly layouts.

interface HelpArticle {
  id: string;
  title: string;
  category: string;
  excerpt: string;
}

// Backend integration stub
const SAMPLE_ARTICLES: HelpArticle[] = [
  { id: '1', title: 'Set up your store in 5 minutes', category: 'Getting Started', excerpt: 'Learn how to add your business name, logo, and hours.' },
  { id: '2', title: 'Accept your first payment', category: 'Payments', excerpt: 'Connect your bank account and start getting paid today.' },
  { id: '3', title: 'What is an AI Support Agent?', category: 'AI Agents', excerpt: 'Let our AI handle basic customer questions for you.' },
];

// In production, this component fetches data from the gRPC backend:
// async function fetchArticles(query: string) {
//   const response = await fetch('/api/help/articles', { method: 'POST', body: JSON.stringify({ query }) });
//   return response.json();
// }

export const HelpCenter: React.FC = () => {
  const [searchQuery, setSearchQuery] = useState('');

  const filteredArticles = SAMPLE_ARTICLES.filter(article =>
    article.title.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div style={{ padding: '20px', maxWidth: '600px', margin: '0 auto', fontFamily: 'Inter, sans-serif' }}>
      <h1 style={{ fontFamily: 'Outfit, sans-serif', fontSize: '24px', marginBottom: '16px' }}>Help Center</h1>
      <p style={{ marginBottom: '24px', color: '#666' }}>Find answers to your questions, simply.</p>

      <input
        type="text"
        placeholder="Search for help..."
        value={searchQuery}
        onChange={(e) => setSearchQuery(e.target.value)}
        style={{ width: '100%', padding: '12px', fontSize: '16px', borderRadius: '8px', border: '1px solid #ccc', marginBottom: '32px' }}
      />

      <div>
        {filteredArticles.length === 0 ? (
          <p>No results found for "{searchQuery}". Try another word.</p>
        ) : (
          filteredArticles.map(article => (
            <div key={article.id} style={{ marginBottom: '20px', padding: '16px', border: '1px solid #eee', borderRadius: '8px', background: '#fafafa' }}>
              <div style={{ fontSize: '12px', color: '#888', marginBottom: '4px', textTransform: 'uppercase', letterSpacing: '1px' }}>{article.category}</div>
              <h3 style={{ fontSize: '18px', margin: '0 0 8px 0', color: '#0056b3' }}>{article.title}</h3>
              <p style={{ margin: '0', color: '#444' }}>{article.excerpt}</p>
            </div>
          ))
        )}
      </div>
    </div>
  );
};
