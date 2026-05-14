import React from 'react';
import HelpCenter from '../../components/help/HelpCenter';
import VideoTutorials from '../../components/help/VideoTutorials';
import ReleaseNotes from '../../components/help/ReleaseNotes';
import ApiDocs from '../../components/help/ApiDocs';

export default function HelpPage() {
    return (
        <div style={{ backdropFilter: 'blur(20px) saturate(200%)', minHeight: '100vh', background: '#f8fafc' }}>
            <div style={{ maxWidth: '1200px', margin: '0 auto', padding: '40px 20px' }}>
                <HelpCenter />
                <hr style={{ margin: '40px 0', border: 'none', borderTop: '1px solid #e2e8f0' }} />
                <VideoTutorials />
                <hr style={{ margin: '40px 0', border: 'none', borderTop: '1px solid #e2e8f0' }} />
                <ReleaseNotes />
                <hr style={{ margin: '40px 0', border: 'none', borderTop: '1px solid #e2e8f0' }} />
                <ApiDocs />
            </div>
        </div>
    );
}
