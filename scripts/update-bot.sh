#!/usr/bin/env bash
set -euo pipefail

# ─── Configuration (override with env vars if you like) ───────────────────────

GHCR_USER=${GHCR_USER:-premiertutors}           # your GHCR org/user
IMAGE_NAME=${IMAGE_NAME:-typst-bot}             # GHCR image repo
IMAGE_TAG=${IMAGE_TAG:-main}                    # tag for the HTTP-server build

DROPLET_IP=${DROPLET_IP:?Please export DROPLET_IP}
SSH_USER=${SSH_USER:-root}
SSH_KEY_PATH=${SSH_KEY_PATH:-~/.ssh/deploy_key}

PACKAGE_NAMESPACE=${PACKAGE_NAMESPACE:?Please export PACKAGE_NAMESPACE}
PACKAGE_NAME=${PACKAGE_NAME:?Please export PACKAGE_NAME}
PACKAGE_REPO_URL=${PACKAGE_REPO_URL:?Please export PACKAGE_REPO_URL}

# ─── 1) Determine latest package version ──────────────────────────────────────
echo "📦 Fetching latest tag from ${PACKAGE_REPO_URL}…"
LATEST_TAG=$(git ls-remote --tags --sort="v:refname" "${PACKAGE_REPO_URL}" \
  | tail -n1 \
  | sed 's|.*refs/tags/||')
if [[ -z "$LATEST_TAG" ]]; then
  echo "⚠️  No tags found; defaulting to main branch."
  LATEST_TAG="main"
fi
echo "✅ Latest package version is ${LATEST_TAG}"

# ─── 2) Log in to GHCR ────────────────────────────────────────────────────────
echo "${GHCR_PAT:?Please export GHCR_PAT}" \
  | docker login ghcr.io --username "${GHCR_USER}" --password-stdin

# ─── 3) Build & push the HTTP-server image ────────────────────────────────────
docker build -f Dockerfile.http \
  -t ghcr.io/${GHCR_USER}/${IMAGE_NAME}:http-server-${IMAGE_TAG} .
docker push ghcr.io/${GHCR_USER}/${IMAGE_NAME}:http-server-${IMAGE_TAG}

# ─── 4) SSH into droplet, sync package, pull & restart ───────────────────────
ssh -i "${SSH_KEY_PATH}" ${SSH_USER}@${DROPLET_IP} <<EOF
  set -e
  cd /opt/typst-bot

  PKG_DIR=cache/${PACKAGE_NAMESPACE}/${PACKAGE_NAME}

  if [ -d "\$PKG_DIR" ]; then
    echo "🔄 Updating existing package…"
    cd "\$PKG_DIR"
    git fetch origin --tags
    git checkout ${LATEST_TAG}
  else
    echo "📥 Cloning package for the first time…"
    git clone --branch ${LATEST_TAG} ${PACKAGE_REPO_URL} "\$PKG_DIR"
  fi

  # Pull & restart
  docker-compose -f docker-compose.http.yml pull
  docker-compose -f docker-compose.http.yml up -d --remove-orphans
EOF

echo "🎉 Done! Bot is now running ghcr.io/${GHCR_USER}/${IMAGE_NAME}:http-server-${IMAGE_TAG}"
