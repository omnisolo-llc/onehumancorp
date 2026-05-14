import React, { useState, useEffect } from 'react';

interface VideoPlayerProps {
    videoId: string;
    title: string;
}

export const VideoPlayer: React.FC<VideoPlayerProps> = ({ videoId, title }) => {
    // In a real implementation, we would fetch metadata from the backend
    const [metadata, setMetadata] = useState<{ url: string; duration: string } | null>(null);

    useEffect(() => {
        // Mock backend fetch
        setTimeout(() => {
            setMetadata({
                url: `https://example.com/videos/${videoId}.mp4`,
                duration: "1:30"
            });
        }, 500);
    }, [videoId]);

    return (
        <div style={{
            maxWidth: '100%',
            borderRadius: '12px',
            overflow: 'hidden',
            boxShadow: '0 4px 15px rgba(0,0,0,0.1)',
            fontFamily: 'Inter, sans-serif'
        }}>
            <div style={{ padding: '15px', backgroundColor: '#f8f9fa', borderBottom: '1px solid #eaeaea', display: 'flex', justifyContent: 'space-between' }}>
                <h4 style={{ margin: 0, fontSize: '16px', color: '#333' }}>{title}</h4>
                {metadata && <span style={{ fontSize: '12px', color: '#888' }}>{metadata.duration}</span>}
            </div>
            <div style={{
                position: 'relative',
                paddingBottom: '56.25%', // 16:9 aspect ratio
                height: 0,
                backgroundColor: '#000'
            }}>
                <div style={{
                    position: 'absolute',
                    top: '50%',
                    left: '50%',
                    transform: 'translate(-50%, -50%)',
                    color: 'white',
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    gap: '10px'
                }}>
                    <div style={{
                        width: '50px',
                        height: '50px',
                        borderRadius: '50%',
                        backgroundColor: 'rgba(255,255,255,0.2)',
                        display: 'flex',
                        justifyContent: 'center',
                        alignItems: 'center',
                        cursor: 'pointer'
                    }}>
                        ▶
                    </div>
                    <span>Play Tutorial</span>
                </div>
            </div>
        </div>
    );
};
