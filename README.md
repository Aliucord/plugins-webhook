# plugins-webhook

## Running

Environment variables (required):

| Name                 | Type   | Description                                                     |
|----------------------|--------|-----------------------------------------------------------------|
| `WEBHOOK_SECRET`     | string | The secret set in the webhook dashboard to verify requests.     |
| `PLUGINS_REPO`       | string | Pointer to the plugins repository (ex. `Aliucord/plugins`)      |
| `PLUGINS_REPO_TOKEN` | string | A Github PAT that has the `workflow` scope to the plugins repo. |

Endpoints (listening on port 80):

- `/github` target endpoint for the GitHub webhook
