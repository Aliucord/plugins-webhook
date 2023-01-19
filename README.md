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

### Docker

Only the following arch's have prebuit images: amd64/x86_64, arm64

```shell
$ docker run -d \
	-p xxxx:80 \
	--restart unless-stopped \
	--name plugins-webhook
	-e PLUGINS_REPO=xxxx/xxxx \
	-e PLUGINS_REPO_TOKEN=xxxx \
	-e WEBHOOK_SECRET=xxxx \
	ghcr.io/aliucord/plugins-webhook:latest
```
