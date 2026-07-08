import React, { useState } from 'react';
import { Box, Text, useInput } from 'ink';

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
  "Browse Agent Marketplace",
  "Visual Workflow Builder",
  "Verify Setup",
  "Exit"
];

interface MasterMenuProps { onSelect?: (option: string) => void; }
export const MasterMenu: React.FC<MasterMenuProps> = ({ onSelect }) => {
  const [selectedIndex, setSelectedIndex] = useState(0);

  useInput((input, key) => {
    if (key.upArrow) {
      setSelectedIndex(prev => Math.max(0, prev - 1));
    }
    if (key.downArrow) {
      setSelectedIndex(prev => Math.min(options.length - 1, prev + 1));
    }
    if (key.return) {
      if (options[selectedIndex] === "Exit") {
        process.exit(0);
      } else {
        if (onSelect) onSelect(options[selectedIndex]);
      }
    }
  });

  return (
    <Box flexDirection="column" borderStyle="round" borderColor="gray" padding={1}>
      <Text bold color="cyan" >Select an action (Use Up/Down arrows):</Text>
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
