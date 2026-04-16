import React from 'react';
import TaskDAGViewer from '../components/orchestration/TaskDAGViewer';
import TeammateMeshConsole from '../components/orchestration/TeammateMeshConsole';
import SwarmOverview from '../components/orchestration/SwarmOverview';
import AutoDreamPipelineWidget from '../components/orchestration/AutoDreamPipelineWidget';
import BusinessSetupWizard from '../components/wizard/BusinessSetupWizard';

const Dashboard = () => {
  return (
    <div style={{ padding: '40px', background: '#111', minHeight: '100vh', fontFamily: '"Outfit", "Inter", sans-serif' }}>
      <h1 style={{ color: '#fff', marginBottom: '30px' }}>Swarm Orchestration Dashboard</h1>
      <BusinessSetupWizard />
      <AutoDreamPipelineWidget />
      <SwarmOverview />
      <TaskDAGViewer />
      <TeammateMeshConsole />
    </div>
  );
};

export default Dashboard;
