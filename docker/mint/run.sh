#!/bin/sh

# Example Usage:
# ./run.sh <IMAGE> <AC_HOST> <AC_PORT> <LOG_LEVEL>
# ./run.sh libra_mint:latest ac.dev.aws.hlw3truzy4ls.com 80 info

set -ex

IMAGE="${1:-libra_mint:latest}"
AC_HOST="${2:-172.18.0.13}"
AC_PORT="${3:-30307}"
LOG_LEVEL="${4:-info}"
CONFIGDIR="$(dirname "$0")/../../terraform/validator-sets/dev"

docker network create --subnet 172.18.0.0/24 testnet || true

docker run -e AC_HOST="$AC_HOST" -e AC_PORT="$AC_PORT" -e LOG_LEVEL="$LOG_LEVEL" \
	-v "$PWD/$CONFIGDIR/trusted_peers.config.toml:/opt/libra/etc/trusted_peers.config.toml" \
	-e MINT_KEY="$(base64 $CONFIGDIR/mint.key)" \
	--network testnet --publish 8080:8000 "$IMAGE"
