# Auto-Deployment Setup

This setup automatically triggers deployment of the typst-bot when a new release is published in the 2_typst repository using the existing release workflow.

## Setup Instructions

### 1. Create GitHub App (You've already done this!)

You mentioned you've set up a GitHub private app with a private key. Great!

### 2. Configure the GitHub App

Your GitHub app needs the following permissions:

- **Repository permissions:**
  - Actions: Write (to trigger workflows)
  - Metadata: Read

- **Organization permissions:**
  - None required

### 3. Install the GitHub App

Install your GitHub app on both repositories:

- `premiertutors/2_typst`
- `premiertutors/typst-bot-pt`

### 4. Add Secrets

#### In `premiertutors/2_typst` repository

Add a repository secret named `DEPLOY_TRIGGER_TOKEN` with the value being either:

- Your GitHub App's private key (if using app authentication)
- A Personal Access Token with `repo` scope (simpler option)

#### In `premiertutors/typst-bot-pt` repository

Your existing secrets should already be configured:

- `REPO_B_SSH_KEY`
- `SSH_DEPLOY_KEY`
- `DROPLET_IP`

### 5. How it Works

```text
Create Release PR → Merge PR → Release Published
                                      ↓
                         Modified release.yml workflow runs
                                      ↓  
                    Sends repository_dispatch to typst-bot-pt
                                      ↓
                            deploy.yml workflow runs
                                      ↓
                           Deploys the new release
```

### 6. Testing

1. Create a release using the existing release workflow in `2_typst` repository
2. Merge the release PR
3. Check that the `release.yml` workflow completes and triggers deployment
4. Check the `typst-bot-pt` repository Actions tab to see if `deploy.yml` is triggered
5. Verify the deployment completes successfully

### 7. Manual Override

You can still manually trigger deployments using the "Run workflow" button in the `deploy.yml` workflow.

## Payload Structure

The repository dispatch includes this payload:
```json
{
  "tag_name": "v1.2.3",
  "release_url": "https://github.com/premiertutors/2_typst/releases/tag/v1.2.3",
  "release_name": "Version 1.2.3",
  "repository": "premiertutors/2_typst"
}
```

## Troubleshooting

- **Workflow not triggering:** Check that the `DEPLOY_TRIGGER_TOKEN` secret is correctly set
- **Authentication errors:** Verify your GitHub App has the correct permissions and is installed on both repos
- **Deployment failures:** Check the deploy.yml workflow logs for specific error messages
