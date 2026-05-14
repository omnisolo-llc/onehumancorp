import React, { useState } from 'react';
import { Box, Text } from 'ink';

export const HelpCenter = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [activeTab, setActiveTab] = useState('home');

  if (!isOpen) {
    return (
      <Box padding={1} borderStyle="round" borderColor="yellow" flexDirection="row" justifyContent="space-between">
        <Text color="yellow">Need help? Press ? to open the Help Center</Text>
      </Box>
    );
  }

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="cyan" padding={1} width={80}>
      <Box justifyContent="center" marginBottom={1}>
        <Text bold color="white" backgroundColor="cyan"> OHC HELP CENTER </Text>
      </Box>
      <Box flexDirection="row" marginBottom={1}>
        <Text color={activeTab === 'home' ? 'green' : 'white'}>[ Getting Started ] </Text>
        <Text color={activeTab === 'payments' ? 'green' : 'white'}>[ Payments ] </Text>
        <Text color={activeTab === 'agents' ? 'green' : 'white'}>[ AI Agents ] </Text>
      </Box>

      <Box borderStyle="single" borderColor="gray" padding={1} flexDirection="column">
        {activeTab === 'home' && (
          <Text>Welcome! We are so glad you are here. Starting a new business is a big step, but you do not need to be a computer expert to do it. Use the tabs above to explore more topics.

          Tip: Look for (?) tooltips around the app for quick hints!</Text>
        )}
        {activeTab === 'payments' && (
          <Text>Getting paid should be the easiest part of running your business. With One Human Corp, you can securely accept credit cards in just a few minutes. Go to Settings {'>'} Payments to connect your bank account.</Text>
        )}
        {activeTab === 'agents' && (
          <Text>Think of an AI Agent as a very smart, very fast employee who works 24/7 and never gets tired. You can turn on the Support Agent in Settings {'>'} My Team to have it automatically answer customer questions.</Text>
        )}
      </Box>
      <Box marginTop={1}>
        <Text color="gray">Press Esc to close • Type "Ask Agent: [your question]" below to talk to the AI Help Agent</Text>
      </Box>
    </Box>
  );
};
