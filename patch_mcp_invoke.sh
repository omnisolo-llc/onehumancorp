#!/bin/bash

# Define the file to patch
FILE="srcs/server/dashboard/handlers_mcp.go"

# We want to replace this block:
# 	result, err := s.invokeMCPTool(req)
#
# 	s.mu.Lock()
# 	if err != nil {

awk '
/result, err := s.invokeMCPTool\(req\)/ {
    print "	result, err := s.invokeMCPTool(req)"
    print ""
    print "	if err == nil && req.HybridEscalation {"
    print "		var paramsMap map[string]interface{}"
    print "		json.Unmarshal(req.Params, &paramsMap)"
    print "		redactedParams := telemetry.RedactInterfacePII(paramsMap)"
    print "		redactedResult := telemetry.RedactInterfacePII(result)"
    print "		missionID := \"mcp-sync-\" + time.Now().UTC().Format(\"20060102150405.999999999\")"
    print "		payloadMap := map[string]interface{}{"
    print "			\"toolId\": req.ToolID,"
    print "			\"action\": req.Action,"
    print "			\"params\": redactedParams,"
    print "			\"result\": redactedResult,"
    print "			\"escalation\": true,"
    print "		}"
    print "		payloadBytes, _ := json.Marshal(payloadMap)"
    print "		if s.hub.SIPDB() != nil {"
    print "			_ = s.hub.SIPDB().UpsertMission(r.Context(), missionID, \"CLOUD_ESCALATION\", string(payloadBytes), false)"
    print "		}"
    print "	}"
    print ""
    print "	s.mu.Lock()"
    print "	if err != nil {"
    found = 1
    next
}
{
    if (found == 1) {
        if ($0 == "	s.mu.Lock()") {
            next
        }
        if ($0 == "	if err != nil {") {
            found = 0
            next
        }
    }
    print $0
}
' $FILE > ${FILE}.tmp && mv ${FILE}.tmp $FILE
