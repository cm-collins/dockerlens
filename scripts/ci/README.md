# CI Bootstrap

This folder contains Azure DevOps bootstrap helpers for DockerLens.

## Managed DevOps Pool bootstrap

Use `bootstrap-managed-devops-pool.sh` to create the missing Azure resources required for a Managed DevOps Pool:

- Azure resource group
- `Microsoft.DevCenter` provider registration
- `Microsoft.DevOpsInfrastructure` provider registration
- Dev Center
- Dev Center project
- Managed DevOps Pool

### Usage

```bash
cp scripts/ci/managed-devops-pool.env.example scripts/ci/managed-devops-pool.env
# edit the values in managed-devops-pool.env

bash scripts/ci/bootstrap-managed-devops-pool.sh
```

### Notes

- The script is idempotent: if a resource already exists, it leaves it in place.
- `az`, the `devcenter` extension, and the `mdp` extension are required. The script installs the extensions automatically, but it expects Azure CLI itself to already be installed and logged in.
- The Azure DevOps organization URL must already exist and be connected correctly for Managed DevOps Pools.
