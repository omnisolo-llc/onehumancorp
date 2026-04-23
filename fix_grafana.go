package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

func main() {
	filePath := "deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json"
	content, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	var dashboard map[string]interface{}
	err = json.Unmarshal(content, &dashboard)
	if err != nil {
		fmt.Println("Error unmarshaling json:", err)
		return
	}

	panels := dashboard["panels"].([]interface{})

	// Add MemoriesProcessedTotal Panel
	memoriesProcessedPanel := map[string]interface{}{
		"type": "timeseries",
		"title": "AutoDream Memories Processed",
		"gridPos": map[string]interface{}{
			"h": 8,
			"w": 12,
			"x": 0,
			"y": 19,
		},
		"id": 5,
		"transparent": true,
		"datasource": map[string]interface{}{
			"type": "prometheus",
			"uid": "Prometheus",
		},
		"options": map[string]interface{}{
			"legend": map[string]interface{}{
				"calcs": []interface{}{},
				"displayMode": "list",
				"placement": "bottom",
				"showLegend": true,
			},
			"tooltip": map[string]interface{}{
				"mode": "multi",
				"sort": "none",
			},
		},
		"targets": []interface{}{
			map[string]interface{}{
				"datasource": map[string]interface{}{
					"type": "prometheus",
					"uid": "Prometheus",
				},
				"editorMode": "code",
				"expr": "sum(rate(ohc_autodream_memories_processed_total[5m])) by (mode, source_type, status)",
				"legendFormat": "{{mode}} - {{source_type}} ({{status}})",
				"range": true,
				"refId": "A",
			},
		},
		"fieldConfig": map[string]interface{}{
			"defaults": map[string]interface{}{
				"color": map[string]interface{}{"mode": "palette-classic"},
				"custom": map[string]interface{}{
					"axisBorderShow": false,
					"axisCenteredZero": false,
					"axisColorMode": "text",
					"axisLabel": "",
					"axisPlacement": "auto",
					"barAlignment": 0,
					"drawStyle": "line",
					"fillOpacity": 10,
					"gradientMode": "opacity",
					"hideFrom": map[string]interface{}{"legend": false, "tooltip": false, "viz": false},
					"insertNulls": false,
					"lineInterpolation": "smooth",
					"lineWidth": 2,
					"pointSize": 5,
					"scaleDistribution": map[string]interface{}{"type": "linear"},
					"showPoints": "auto",
					"spanNulls": false,
					"stacking": map[string]interface{}{"group": "A", "mode": "none"},
					"thresholdsStyle": map[string]interface{}{"mode": "off"},
				},
				"mappings": []interface{}{},
				"thresholds": map[string]interface{}{
					"mode": "absolute",
					"steps": []interface{}{map[string]interface{}{"color": "green", "value": nil}},
				},
			},
			"overrides": []interface{}{},
		},
	}

	// Add BatchProcessingDuration Panel
	batchProcessingPanel := map[string]interface{}{
		"type": "timeseries",
		"title": "AutoDream Batch Processing Duration (P95)",
		"gridPos": map[string]interface{}{
			"h": 8,
			"w": 12,
			"x": 12,
			"y": 19,
		},
		"id": 6,
		"transparent": true,
		"datasource": map[string]interface{}{
			"type": "prometheus",
			"uid": "Prometheus",
		},
		"options": map[string]interface{}{
			"legend": map[string]interface{}{
				"calcs": []interface{}{},
				"displayMode": "list",
				"placement": "bottom",
				"showLegend": true,
			},
			"tooltip": map[string]interface{}{
				"mode": "multi",
				"sort": "none",
			},
		},
		"targets": []interface{}{
			map[string]interface{}{
				"datasource": map[string]interface{}{
					"type": "prometheus",
					"uid": "Prometheus",
				},
				"editorMode": "code",
				"expr": "histogram_quantile(0.95, sum(rate(ohc_autodream_batch_processing_duration_seconds_bucket[5m])) by (le, mode, pipeline))",
				"legendFormat": "{{mode}} - {{pipeline}}",
				"range": true,
				"refId": "A",
			},
		},
		"fieldConfig": map[string]interface{}{
			"defaults": map[string]interface{}{
				"color": map[string]interface{}{"mode": "palette-classic"},
				"custom": map[string]interface{}{
					"axisBorderShow": false,
					"axisCenteredZero": false,
					"axisColorMode": "text",
					"axisLabel": "",
					"axisPlacement": "auto",
					"barAlignment": 0,
					"drawStyle": "line",
					"fillOpacity": 10,
					"gradientMode": "opacity",
					"hideFrom": map[string]interface{}{"legend": false, "tooltip": false, "viz": false},
					"insertNulls": false,
					"lineInterpolation": "smooth",
					"lineWidth": 2,
					"pointSize": 5,
					"scaleDistribution": map[string]interface{}{"type": "linear"},
					"showPoints": "auto",
					"spanNulls": false,
					"stacking": map[string]interface{}{"group": "A", "mode": "none"},
					"thresholdsStyle": map[string]interface{}{"mode": "off"},
				},
				"mappings": []interface{}{},
				"thresholds": map[string]interface{}{
					"mode": "absolute",
					"steps": []interface{}{map[string]interface{}{"color": "green", "value": nil}},
				},
				"unit": "s",
			},
			"overrides": []interface{}{},
		},
	}

	// Add ConsolidationErrorsTotal Panel
	errorsPanel := map[string]interface{}{
		"type": "timeseries",
		"title": "AutoDream Consolidation Errors",
		"gridPos": map[string]interface{}{
			"h": 8,
			"w": 24,
			"x": 0,
			"y": 27,
		},
		"id": 7,
		"transparent": true,
		"datasource": map[string]interface{}{
			"type": "prometheus",
			"uid": "Prometheus",
		},
		"options": map[string]interface{}{
			"legend": map[string]interface{}{
				"calcs": []interface{}{},
				"displayMode": "list",
				"placement": "bottom",
				"showLegend": true,
			},
			"tooltip": map[string]interface{}{
				"mode": "multi",
				"sort": "none",
			},
		},
		"targets": []interface{}{
			map[string]interface{}{
				"datasource": map[string]interface{}{
					"type": "prometheus",
					"uid": "Prometheus",
				},
				"editorMode": "code",
				"expr": "sum(rate(ohc_autodream_consolidation_errors_total[5m])) by (mode, pipeline, error_type)",
				"legendFormat": "{{mode}} - {{pipeline}} ({{error_type}})",
				"range": true,
				"refId": "A",
			},
		},
		"fieldConfig": map[string]interface{}{
			"defaults": map[string]interface{}{
				"color": map[string]interface{}{"mode": "palette-classic"},
				"custom": map[string]interface{}{
					"axisBorderShow": false,
					"axisCenteredZero": false,
					"axisColorMode": "text",
					"axisLabel": "",
					"axisPlacement": "auto",
					"barAlignment": 0,
					"drawStyle": "line",
					"fillOpacity": 10,
					"gradientMode": "opacity",
					"hideFrom": map[string]interface{}{"legend": false, "tooltip": false, "viz": false},
					"insertNulls": false,
					"lineInterpolation": "smooth",
					"lineWidth": 2,
					"pointSize": 5,
					"scaleDistribution": map[string]interface{}{"type": "linear"},
					"showPoints": "auto",
					"spanNulls": false,
					"stacking": map[string]interface{}{"group": "A", "mode": "none"},
					"thresholdsStyle": map[string]interface{}{"mode": "off"},
				},
				"mappings": []interface{}{},
				"thresholds": map[string]interface{}{
					"mode": "absolute",
					"steps": []interface{}{map[string]interface{}{"color": "green", "value": nil}},
				},
			},
			"overrides": []interface{}{},
		},
	}

	panels = append(panels, memoriesProcessedPanel, batchProcessingPanel, errorsPanel)
	dashboard["panels"] = panels

	updatedContent, err := json.MarshalIndent(dashboard, "", "  ")
	if err != nil {
		fmt.Println("Error marshaling json:", err)
		return
	}

	err = ioutil.WriteFile(filePath, updatedContent, 0644)
	if err != nil {
		fmt.Println("Error writing file:", err)
		return
	}

	fmt.Println("Grafana dashboard successfully patched")
}
