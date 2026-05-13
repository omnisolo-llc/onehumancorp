import React from 'react';
import ReferralCard from './ReferralCard';
import SocialAutoPost from './SocialAutoPost';
import EmailMarketing from './EmailMarketing';
import Milestones from './Milestones';
import StorefrontShare from './StorefrontShare';

export default function GrowthDashboard() {
    return (
        <div className="p-6 space-y-6">
            <h1 className="text-3xl font-bold text-gray-900">Growth Engine</h1>
            <p className="text-gray-600">Grow your OneHumanCorp business.</p>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <ReferralCard />
                <SocialAutoPost />
                <EmailMarketing />
                <Milestones />
                <StorefrontShare />
            </div>
        </div>
    );
}
