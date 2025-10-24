#!/bin/sh

if ! test -e $PWD/run.sh; then
  echo "Run this from the fizz bot's root directory."
  exit 1
fi

MODE=$1
if [[ "$MODE" == "prod" ]]; then
  echo "PROD mode"
  export FIZZ_CONFIG_DIR=$PWD/prod
  export DISCORD_TOKEN=`sh prod/SECRETS`
elif [[ "$MODE" == "staging" ]]; then
  echo "STAGING mode"
  export FIZZ_CONFIG_DIR=$PWD/staging
  export DISCORD_TOKEN=`sh staging/SECRETS`
else
  echo "Specify prod or staging mode."
  exit 1
fi

echo "Using config $FIZZ_CONFIG_DIR/fizz.toml"
cargo run
