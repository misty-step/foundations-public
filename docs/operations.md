# Operations

Public-safe summary of how this repository is operated. Fleet-specific procedures (hosts, credential stores, staffing and account routing) are internal to Misty Step and are not published here. The [Constitution](https://foundations.mistystep.io/) and [Way of working](https://foundations.mistystep.io/way-of-working) own lasting engineering principles.

## Backlog routing

Current work lives in this repository's GitHub issues. Do not keep a parallel queue elsewhere.

## CI deploy credentials

- Deployment uses a narrowly scoped Cloudflare API token stored as the repository secret `CLOUDFLARE_API_TOKEN`, plus the `CLOUDFLARE_ACCOUNT_ID` repository variable. Only maintainers set them. Never print the values.
- Account-level or over-broad tokens never go into CI, a Worker, or a file.
- Pull requests, including from forks, never receive these secrets. Only the reviewed `master` branch deploys.
