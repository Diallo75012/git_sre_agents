---
# SRE2 Project Notes

## Nginx Deployment Overview

### ConfigMap
- **File**: nginx_configmap.yaml
- **Purpose**: Stores Nginx configuration files (e.g., default server block, custom SSL certs, reverse proxy rules).
- **When to update**: When modifying server behavior, enabling modules, or updating static content.

### Deployment
- **File**: nginx_deployment.yaml
- **Settings**: 
  - Image version (ensure it's pinned in production).
  - Resource limits (CPU/Memory) to prevent over/under allocation.
  - Readiness/Liveness probes for health checks.
- **Replicas**: Defined in the specification; adjust based on expected load and scalability needs.

### Service
- **File**: nginx_service.yaml
- **Type**: ClusterIP (current setup, exposes internally to the cluster).
- **Ports**: Ensures proper mapping between container and service ports.

## Best Practices

1. **Version Control**: Keep all YAML files under Git to track changes and rollbacks.
2. **Secret Management**: Use Kubernetes Secrets for SSL certificates or private configs instead of plaintext in ConfigMaps.
3. **Testing**: Validate config changes and deployment updates in staging before production.
4. **Automated Rollouts**: Utilize Helm charts or kubectl apply for controlled version updates.

## Troubleshooting
- Use `kubectl describe pod <nginx-pod-name>` for pod event logs.
- Use `kubectl logs <nginx-pod-name>` to debug container-specific issues.
- Validate configmap correctness: `kubectl get configmap nginx-config -o yaml`. 
- Ensure Nginx workload is scaled to match traffic: `kubectl get deployments/nginx-deployment`.

---
Text content of sre2_notes.md file
