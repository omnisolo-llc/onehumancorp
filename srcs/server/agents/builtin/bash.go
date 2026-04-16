package builtin

import (
	"context"
	"encoding/json"
	"os/exec"
	"time"
	"fmt"
	"os"
)

// BashTool definition
var BashTool = Tool{
	Name:        "Bash",
	Description: "Execute a bash script. " +
		"If run_in_background is true, it returns immediately and you will be notified when it completes.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"command": {
				"type": "string",
				"description": "The bash command or script to execute."
			},
			"run_in_background": {
				"type": "boolean",
				"description": "Set to true to run this command in the background. Use Read to read the output later."
			}
		},
		"required": ["command"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Command         string `json:"command"`
			RunInBackground *bool  `json:"run_in_background"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		isAsync := false
		if input.RunInBackground != nil && *input.RunInBackground {
			isAsync = true
		}

		if isAsync {
			// Generate a task ID to track background bash commands.
			taskID := fmt.Sprintf("b%d", time.Now().UnixNano())

			// Create a secure temporary file
			tmpFile, tmpErr := os.CreateTemp("", fmt.Sprintf("bash_%s_*.out", taskID))
			outputFile := ""
			if tmpErr == nil {
				outputFile = tmpFile.Name()
				tmpFile.Close() // Close it now, we'll write to it later
			} else {
				// Fallback if CreateTemp fails (shouldn't happen in normal environments)
				outputFile = fmt.Sprintf("/tmp/bash_%s.out", taskID)
			}

			go func() {
				// Run in background with a detached context but a timeout to prevent leaks
				bgCtx, cancel := context.WithTimeout(context.Background(), 1*time.Hour)
				defer cancel()

				cmd := exec.CommandContext(bgCtx, "bash", "-c", input.Command)

				// Open file for live streaming
				outFile, err := os.OpenFile(outputFile, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0644)
				if err == nil {
					cmd.Stdout = outFile
					cmd.Stderr = outFile
					defer outFile.Close()
				}

				// Run the command
				err = cmd.Run()

				status := "completed"
				if err != nil {
					status = "failed"
					if outFile != nil {
						outFile.WriteString("\n" + err.Error())
					}
				}

				// Publish notification so the agent knows it finished
				summary := fmt.Sprintf("Background Bash command (Task %s) finished with status %s", taskID, status)
				notif := BuildTaskNotificationMsg(
					taskID, "", outputFile,
					status, summary, "Output written to " + outputFile,
					0, 0,
					0, // duration
				)
				globalSubagentBus.Publish(SubagentLifecycleEvent{
					EventType:    SubagentEventCompleted,
					TaskID:       taskID,
					Notification: notif,
				})
			}()

			return fmt.Sprintf("Command launched in background. Task ID: %s. Output will be written to: %s", taskID, outputFile), nil
		}

		// Synchronous execution
		cmd := exec.CommandContext(ctx, "bash", "-c", input.Command)
		out, err := cmd.CombinedOutput()
		if err != nil {
			return string(out) + "\n" + err.Error(), nil // Returning error as content to the LLM
		}

		return string(out), nil
	},
}