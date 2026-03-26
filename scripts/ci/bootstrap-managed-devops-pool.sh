#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEFAULT_ENV_FILE="${SCRIPT_DIR}/managed-devops-pool.env"
EXAMPLE_ENV_FILE="${SCRIPT_DIR}/managed-devops-pool.env.example"

if [[ -f "${DEFAULT_ENV_FILE}" ]]; then
  # shellcheck disable=SC1090
  source "${DEFAULT_ENV_FILE}"
fi

required_vars=(
  AZURE_SUBSCRIPTION_ID
  AZURE_RESOURCE_GROUP
  AZURE_LOCATION
  AZURE_DEV_CENTER_NAME
  AZURE_DEV_CENTER_PROJECT_NAME
  AZURE_DEVOPS_ORG_URL
)

for var_name in "${required_vars[@]}"; do
  if [[ -z "${!var_name:-}" ]]; then
    echo "Missing required variable: ${var_name}" >&2
    if [[ -f "${EXAMPLE_ENV_FILE}" ]]; then
      echo "Create ${DEFAULT_ENV_FILE} from ${EXAMPLE_ENV_FILE} and fill in the values." >&2
    fi
    exit 1
  fi
done

POOL_NAME="${AZURE_MANAGED_POOL_NAME:-dockerlens-dev}"
MAX_CONCURRENCY="${AZURE_MANAGED_POOL_MAX_CONCURRENCY:-1}"
POOL_VM_SKU="${AZURE_MANAGED_POOL_VM_SKU:-Standard_D2as_v5}"
POOL_IMAGE_ALIAS="${AZURE_MANAGED_POOL_IMAGE_ALIAS:-ubuntu-24.04}"
POOL_IMAGE_WELL_KNOWN="${AZURE_MANAGED_POOL_IMAGE_WELL_KNOWN:-ubuntu-24.04/latest}"
AZURE_DEVOPS_PROJECTS="${AZURE_DEVOPS_PROJECTS:-}"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Required command not found: $1" >&2
    exit 1
  fi
}

create_temp_json() {
  local path="$1"
  local content="$2"
  printf '%s\n' "${content}" > "${path}"
}

require_command az

if ! az account show >/dev/null 2>&1; then
  echo "Azure CLI is not logged in. Run 'az login' first." >&2
  exit 1
fi

echo "Preparing Azure CLI extensions..."
az extension add --name devcenter --upgrade >/dev/null
az extension add --name mdp --upgrade >/dev/null

echo "Selecting subscription ${AZURE_SUBSCRIPTION_ID}..."
az account set --subscription "${AZURE_SUBSCRIPTION_ID}"

echo "Registering required Azure resource providers..."
az provider register --namespace Microsoft.DevCenter --wait >/dev/null
az provider register --namespace Microsoft.DevOpsInfrastructure --wait >/dev/null

echo "Ensuring resource group ${AZURE_RESOURCE_GROUP} exists..."
az group create \
  --name "${AZURE_RESOURCE_GROUP}" \
  --location "${AZURE_LOCATION}" \
  >/dev/null

echo "Ensuring dev center ${AZURE_DEV_CENTER_NAME} exists..."
if ! az devcenter admin devcenter show \
  --name "${AZURE_DEV_CENTER_NAME}" \
  --resource-group "${AZURE_RESOURCE_GROUP}" \
  >/dev/null 2>&1; then
  az devcenter admin devcenter create \
    --name "${AZURE_DEV_CENTER_NAME}" \
    --resource-group "${AZURE_RESOURCE_GROUP}" \
    --location "${AZURE_LOCATION}" \
    >/dev/null
fi

DEV_CENTER_ID="$(
  az devcenter admin devcenter show \
    --name "${AZURE_DEV_CENTER_NAME}" \
    --resource-group "${AZURE_RESOURCE_GROUP}" \
    --query id \
    --output tsv
)"

echo "Ensuring dev center project ${AZURE_DEV_CENTER_PROJECT_NAME} exists..."
if ! az devcenter admin project show \
  --name "${AZURE_DEV_CENTER_PROJECT_NAME}" \
  --resource-group "${AZURE_RESOURCE_GROUP}" \
  >/dev/null 2>&1; then
  az devcenter admin project create \
    --name "${AZURE_DEV_CENTER_PROJECT_NAME}" \
    --resource-group "${AZURE_RESOURCE_GROUP}" \
    --location "${AZURE_LOCATION}" \
    --description "Managed DevOps Pool project for DockerLens" \
    --dev-center-id "${DEV_CENTER_ID}" \
    >/dev/null
fi

DEV_CENTER_PROJECT_ID="$(
  az devcenter admin project show \
    --name "${AZURE_DEV_CENTER_PROJECT_NAME}" \
    --resource-group "${AZURE_RESOURCE_GROUP}" \
    --query id \
    --output tsv
)"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "${tmp_dir}"' EXIT

agent_profile_path="${tmp_dir}/agent-profile.json"
fabric_profile_path="${tmp_dir}/fabric-profile.json"
organization_profile_path="${tmp_dir}/organization-profile.json"

create_temp_json "${agent_profile_path}" '{
  "Stateless": {}
}'

create_temp_json "${fabric_profile_path}" "{
  \"vmss\": {
    \"sku\": {
      \"name\": \"${POOL_VM_SKU}\"
    },
    \"images\": [
      {
        \"aliases\": [
          \"${POOL_IMAGE_ALIAS}\"
        ],
        \"buffer\": \"*\",
        \"wellKnownImageName\": \"${POOL_IMAGE_WELL_KNOWN}\"
      }
    ],
    \"osProfile\": {
      \"secretsManagementSettings\": {
        \"observedCertificates\": [],
        \"keyExportable\": false
      },
      \"logonType\": \"Service\"
    },
    \"storageProfile\": {
      \"osDiskStorageAccountType\": \"Standard\",
      \"dataDisks\": []
    }
  }
}"

organization_projects_json='[]'
if [[ -n "${AZURE_DEVOPS_PROJECTS}" ]]; then
  organization_projects_json="$(
    python3 - <<'PY'
import json
import os

projects = [p.strip() for p in os.environ["AZURE_DEVOPS_PROJECTS"].split(",") if p.strip()]
print(json.dumps(projects))
PY
  )"
fi

create_temp_json "${organization_profile_path}" "{
  \"AzureDevOps\": {
    \"organizations\": [
      {
        \"url\": \"${AZURE_DEVOPS_ORG_URL}\",
        \"projects\": ${organization_projects_json},
        \"parallelism\": ${MAX_CONCURRENCY}
      }
    ],
    \"permissionProfile\": {
      \"kind\": \"CreatorOnly\"
    }
  }
}"

echo "Ensuring Managed DevOps Pool ${POOL_NAME} exists..."
if ! az mdp pool show \
  --name "${POOL_NAME}" \
  --resource-group "${AZURE_RESOURCE_GROUP}" \
  >/dev/null 2>&1; then
  az mdp pool create \
    --name "${POOL_NAME}" \
    --resource-group "${AZURE_RESOURCE_GROUP}" \
    --location "${AZURE_LOCATION}" \
    --devcenter-project-id "${DEV_CENTER_PROJECT_ID}" \
    --maximum-concurrency "${MAX_CONCURRENCY}" \
    --agent-profile "${agent_profile_path}" \
    --fabric-profile "${fabric_profile_path}" \
    --organization-profile "${organization_profile_path}" \
    >/dev/null
fi

echo
echo "Managed DevOps Pool is ready."
echo "Pool name: ${POOL_NAME}"
echo "Resource group: ${AZURE_RESOURCE_GROUP}"
echo "Dev center project: ${AZURE_DEV_CENTER_PROJECT_NAME}"
echo
echo "Next step: make sure the Azure pipeline variable AGENT_POOL matches '${POOL_NAME}'."
