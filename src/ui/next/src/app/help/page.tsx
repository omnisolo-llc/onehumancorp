
import React from 'react';
import HelpCenter from '../../components/help/HelpCenter';
import VideoTutorials from '../../components/help/VideoTutorials';
import ReleaseNotes from '../../components/help/ReleaseNotes';
import ApiDocs from '../../components/help/ApiDocs';

export default function HelpPage() {
    return (
        <div style={{ backdropFilter: 'blur(20px) saturate(200%)' }}>
            <HelpCenter />
            <VideoTutorials />
            <ReleaseNotes />
            <ApiDocs />
        </div>
    );
}
