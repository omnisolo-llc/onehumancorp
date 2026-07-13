import React, { useState } from 'react';
import { Box, Text, useInput } from 'ink';
import { useMarketplace } from '../hooks/useMarketplace.js';
import Spinner from 'ink-spinner';
const AnySpinner = Spinner as any;

interface MarketplaceProps {
  onBack: () => void;
}

export const Marketplace: React.FC<MarketplaceProps> = ({ onBack }) => {
  const { agents, loading, error } = useMarketplace();
  const [selectedIndex, setSelectedIndex] = useState(0);

  useInput((_input, key) => {
    if (key.upArrow) {
      setSelectedIndex(prev => Math.max(0, prev - 1));
    }
    if (key.downArrow) {
      setSelectedIndex(prev => Math.min(agents.length - 1, prev + 1));
    }
    if (key.return) {
       // Just go back for now
       onBack();
    }
    if (key.escape || _input === 'q') {
      onBack();
    }
  });

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="green" padding={1}>
      <Text bold color="green">Agent Marketplace (AutoGPT Harness Mechanic)</Text>

      {loading && (
        <Box marginTop={1}>
          <Text color="yellow"><AnySpinner type="dots" /> Fetching Pre-built Agents...</Text>
        </Box>
      )}

      {error && (
        <Box marginTop={1}>
          <Text color="red">Error: {error}</Text>
        </Box>
      )}

      {!loading && !error && agents.length === 0 && (
        <Box marginTop={1}>
          <Text color="gray">No agents found in the marketplace.</Text>
        </Box>
      )}

      {!loading && !error && agents.length > 0 && (
        <Box flexDirection="column" marginTop={1}>
          {agents.map((agent, index) => {
            const isSelected = index === selectedIndex;
            return (
              <Box key={agent.id} flexDirection="column" marginBottom={1}>
                <Box>
                  <Text color={isSelected ? 'blue' : 'gray'}>
                    {isSelected ? '▶ ' : '  '}
                  </Text>
                  <Text color={isSelected ? 'white' : 'gray'} bold={isSelected}>
                    {agent.name}
                  </Text>
                  <Text color="gray"> (by {agent.author} - {agent.downloads} DLs)</Text>
                </Box>
                {isSelected && (
                  <Box marginLeft={2}>
                    <Text color="dim">{agent.description}</Text>
                  </Box>
                )}
              </Box>
            );
          })}
          <Box marginTop={1}>
            <Text color="gray" italic>Press Enter or 'q' to go back.</Text>
          </Box>
        </Box>
      )}
    </Box>
  );
};
