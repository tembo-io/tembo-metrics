# Tembo Metrics Helm Chart

## Introduction

Tembo Metrics is a Prometheus exporter that provides metrics related to AWS CloudFormation stack usage and quotas. This Helm chart deploys Tembo Metrics in a Kubernetes cluster, making it easy to monitor your CloudFormation resources and set up alerts for quota limits.

## Prerequisites

- Kubernetes 1.16+
- Helm 3.0+
- AWS credentials configured (either through AWS IAM Roles for Service Accounts or environment variables)

## Installing the Chart

To install the chart with the release name `tembo-metrics`:

```bash
helm install tembo-metrics ./tembo-metrics
```

![Version: 0.1.0](https://img.shields.io/badge/Version-0.1.0-informational?style=flat-square) ![Type: application](https://img.shields.io/badge/Type-application-informational?style=flat-square) ![AppVersion: 0.1.0](https://img.shields.io/badge/AppVersion-0.1.0-informational?style=flat-square)

A Helm chart for Tembo Metrics

## Values

| Key                                                       | Type   | Default                                                                              | Description |
| --------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------ | ----------- |
| config.logLevel                                           | string | `"info"`                                                                             |             |
| config.region                                             | string | `"us-east-1"`                                                                        |             |
| config.serverHost                                         | string | `"0.0.0.0"`                                                                          |             |
| config.serverPort                                         | int    | `8080`                                                                               |             |
| image.pullPolicy                                          | string | `"IfNotPresent"`                                                                     |             |
| image.repository                                          | string | `"quay.io/tembo/tembo-metrics"`                                                      |             |
| image.tag                                                 | string | `"latest"`                                                                           |             |
| probes.liveness.failureThreshold                          | int    | `3`                                                                                  |             |
| probes.liveness.httpGet.path                              | string | `"/healthz"`                                                                         |             |
| probes.liveness.httpGet.port                              | string | `"http"`                                                                             |             |
| probes.liveness.initialDelaySeconds                       | int    | `10`                                                                                 |             |
| probes.liveness.periodSeconds                             | int    | `10`                                                                                 |             |
| probes.readiness.failureThreshold                         | int    | `3`                                                                                  |             |
| probes.readiness.httpGet.path                             | string | `"/readyz"`                                                                          |             |
| probes.readiness.httpGet.port                             | string | `"http"`                                                                             |             |
| probes.readiness.initialDelaySeconds                      | int    | `5`                                                                                  |             |
| probes.readiness.periodSeconds                            | int    | `10`                                                                                 |             |
| prometheus.path                                           | string | `"/metrics"`                                                                         |             |
| prometheus.port                                           | int    | `8080`                                                                               |             |
| prometheus.scrape                                         | bool   | `true`                                                                               |             |
| prometheusRules.additionalLabels                          | object | `{}`                                                                                 |             |
| prometheusRules.create                                    | bool   | `true`                                                                               |             |
| prometheusRules.rules[0].name                             | string | `"CloudFormationStackQuota"`                                                         |             |
| prometheusRules.rules[0].rules[0].alert                   | string | `"CloudFormationStackQuotaWarning"`                                                  |             |
| prometheusRules.rules[0].rules[0].annotations.description | string | `"CloudFormation stack usage is at {{ $value }}% of the quota"`                      |             |
| prometheusRules.rules[0].rules[0].annotations.summary     | string | `"CloudFormation stack usage nearing quota"`                                         |             |
| prometheusRules.rules[0].rules[0].expr                    | string | `"(tembo_cloudformation_stack_usage / tembo_cloudformation_stack_quota) * 100 > 90"` |             |
| prometheusRules.rules[0].rules[0].for                     | string | `"5m"`                                                                               |             |
| prometheusRules.rules[0].rules[0].labels.severity         | string | `"warning"`                                                                          |             |
| prometheusRules.rules[0].rules[1].alert                   | string | `"CloudFormationStackQuotaCritical"`                                                 |             |
| prometheusRules.rules[0].rules[1].annotations.description | string | `"CloudFormation stack usage is at {{ $value }}% of the quota"`                      |             |
| prometheusRules.rules[0].rules[1].annotations.summary     | string | `"CloudFormation stack usage critically close to quota"`                             |             |
| prometheusRules.rules[0].rules[1].expr                    | string | `"(tembo_cloudformation_stack_usage / tembo_cloudformation_stack_quota) * 100 > 95"` |             |
| prometheusRules.rules[0].rules[1].for                     | string | `"5m"`                                                                               |             |
| prometheusRules.rules[0].rules[1].labels.severity         | string | `"critical"`                                                                         |             |
| replicaCount                                              | int    | `1`                                                                                  |             |
| resources.limits.cpu                                      | string | `"100m"`                                                                             |             |
| resources.limits.memory                                   | string | `"250Mi"`                                                                            |             |
| resources.requests.cpu                                    | string | `"50m"`                                                                              |             |
| resources.requests.memory                                 | string | `"128Mi"`                                                                            |             |
| service.port                                              | int    | `8080`                                                                               |             |
| service.type                                              | string | `"ClusterIP"`                                                                        |             |
| serviceAccount.annotations                                | object | `{}`                                                                                 |             |
| serviceAccount.create                                     | bool   | `true`                                                                               |             |
| serviceAccount.name                                       | string | `""`                                                                                 |             |

---

Autogenerated from chart metadata using [helm-docs v1.13.1](https://github.com/norwoodj/helm-docs/releases/v1.13.1)
