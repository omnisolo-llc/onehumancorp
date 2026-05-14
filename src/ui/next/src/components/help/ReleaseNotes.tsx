import React from 'react';

// Release Notes Component designed for small business owners
interface Note {
  version: string;
  date: string;
  title: string;
  description: string;
}

const NOTES: Note[] = [
  { version: '1.2.0', date: 'October 24, 2023', title: 'Faster checkouts', description: 'We made the checkout process 20% faster so your customers can complete purchases with fewer clicks.' },
  { version: '1.1.5', date: 'October 10, 2023', title: 'New AI Agent features', description: 'Your AI agent can now automatically send receipts after a successful purchase.' },
];

export const ReleaseNotes: React.FC = () => {
  return (
    <div style={{ padding: '20px', fontFamily: 'Inter, sans-serif', maxWidth: '600px', margin: '0 auto' }}>
      <h2 style={{ fontFamily: 'Outfit, sans-serif', fontSize: '24px', marginBottom: '8px' }}>What's New in OHC</h2>
      <p style={{ color: '#666', marginBottom: '32px' }}>We are always improving to help your business grow.</p>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
        {NOTES.map((note, index) => (
          <div key={index} style={{ borderLeft: '4px solid #0056b3', paddingLeft: '16px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
              <span style={{ fontWeight: 'bold', color: '#0056b3' }}>{note.title}</span>
              <span style={{ fontSize: '12px', color: '#888' }}>{note.date}</span>
            </div>
            <p style={{ margin: '0', color: '#444', lineHeight: '1.5' }}>{note.description}</p>
          </div>
        ))}
      </div>

      <div style={{ marginTop: '32px', textAlign: 'center' }}>
        <a href="/changelog" style={{ color: '#0056b3', textDecoration: 'none', fontWeight: 'bold' }}>Read the full changelog →</a>
      </div>
    </div>
  );
};
