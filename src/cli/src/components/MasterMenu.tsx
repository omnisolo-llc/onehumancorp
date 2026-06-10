import React, { useState } from 'react';
import { Box, Text, useInput } from 'ink';

import { AgentProtocol } from './AgentProtocol';

const options = [
  "Run Developer Setup",
  "Configure Environment (.env)",
  "Run Diagnostics",
  "Launch Quick Start (Standalone)",
  "Provision AI Agent",
  "Standalone DB Health Check",
  "Launch Cloud Start",
  "Seed Database with Mock Data",
  "Check Swarm Status",
  "Verify Setup",
  "View Agent Protocol Tasks",
  "Exit"
];

export const MasterMenu: React.FC = () => {
  const [viewProtocol, setViewProtocol] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);

  useInput((input, key) => {
    if (viewProtocol && key.escape) {
      setViewProtocol(false);
      return;
    }
    if (key.upArrow) {
      setSelectedIndex(prev => Math.max(0, prev - 1));
    }
    if (key.downArrow) {
      setSelectedIndex(prev => Math.min(options.length - 1, prev + 1));
    }
    if (key.return) {
      if (options[selectedIndex] === "View Agent Protocol Tasks") {
        setViewProtocol(true);
      } else if (options[selectedIndex] === "Exit") {
        process.exit(0);
      } else {
        console.info(`Executing ${options[selectedIndex]}...`);
      }
    }
  });

  if (viewProtocol) {
    return (
      <Box flexDirection="column">
        <AgentProtocol />
        <Text color="gray">Press Esc to go back</Text>
      </Box>
    );
  }

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="gray" padding={1}>
      <Text bold color="cyan" marginBottom={1}>Select an action (Use Up/Down arrows):</Text>
      {options.map((option, index) => {
        const isSelected = index === selectedIndex;
        return (
          <Box key={index}>
            <Text color={isSelected ? 'blue' : 'gray'}>
              {isSelected ? '▶ ' : '  '}
            </Text>
            <Text color={isSelected ? 'white' : 'gray'}>
              {index === options.length - 1 ? '0)' : `${index + 1})`} {option}
            </Text>
          </Box>
        );
      })}
    </Box>
  );
};
