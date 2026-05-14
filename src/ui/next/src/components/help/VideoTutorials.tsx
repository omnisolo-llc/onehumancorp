import React from 'react';

// Video Tutorials component for the top 10 most common user tasks.
interface VideoMeta {
  id: string;
  title: string;
  duration: string;
  thumbnailUrl: string;
}

const VIDEOS: VideoMeta[] = [
  { id: 'v1', title: 'How to add a product', duration: '1:15', thumbnailUrl: 'https://via.placeholder.com/300x169?text=Product+Tutorial' },
  { id: 'v2', title: 'Connecting your bank', duration: '0:45', thumbnailUrl: 'https://via.placeholder.com/300x169?text=Bank+Tutorial' },
  { id: 'v3', title: 'Customizing your storefront', duration: '1:20', thumbnailUrl: 'https://via.placeholder.com/300x169?text=Store+Tutorial' },
];

export const VideoTutorials: React.FC = () => {
  return (
    <div style={{ padding: '20px', fontFamily: 'Inter, sans-serif', maxWidth: '800px', margin: '0 auto' }}>
      <h2 style={{ fontFamily: 'Outfit, sans-serif', fontSize: '24px', marginBottom: '8px' }}>Video Tutorials</h2>
      <p style={{ color: '#666', marginBottom: '24px' }}>Short, simple guides to get you up and running quickly.</p>

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(250px, 1fr))', gap: '20px' }}>
        {VIDEOS.map(video => (
          <div key={video.id} style={{ borderRadius: '12px', overflow: 'hidden', border: '1px solid #eee', background: '#fff', cursor: 'pointer', transition: 'transform 0.2s', boxShadow: '0 2px 8px rgba(0,0,0,0.05)' }}>
            <div style={{ position: 'relative' }}>
              <img src={video.thumbnailUrl} alt={video.title} style={{ width: '100%', display: 'block' }} />
              <div style={{ position: 'absolute', bottom: '8px', right: '8px', background: 'rgba(0,0,0,0.8)', color: 'white', padding: '2px 6px', borderRadius: '4px', fontSize: '12px', fontWeight: 'bold' }}>
                {video.duration}
              </div>
            </div>
            <div style={{ padding: '12px' }}>
              <h4 style={{ margin: '0', fontSize: '16px', color: '#333' }}>{video.title}</h4>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
