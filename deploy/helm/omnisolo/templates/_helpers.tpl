{{/*
Expand the name of the chart.
*/}}
{{- define "omnisolo.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "omnisolo.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "omnisolo.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "omnisolo.labels" -}}
helm.sh/chart: {{ include "omnisolo.chart" . }}
{{ include "omnisolo.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "omnisolo.selectorLabels" -}}
app.kubernetes.io/name: {{ include "omnisolo.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Valkey service name from the official valkey chart dependency.
*/}}
{{- define "omnisolo.valkey.fullname" -}}
{{- if .Values.valkey.fullnameOverride }}
{{- .Values.valkey.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default "valkey" .Values.valkey.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{- define "omnisolo.valkey.addr" -}}
{{ include "omnisolo.valkey.fullname" . }}:{{ .Values.valkey.service.port }}
{{- end }}

{{- define "omnisolo.valkey.name" -}}
{{ default "valkey" .Values.valkey.nameOverride }}
{{- end }}

{{- define "omnisolo.valkey.url" -}}
redis://{{ include "omnisolo.valkey.addr" . }}
{{- end }}
