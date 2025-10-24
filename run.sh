#!/bin/sh

MODE=$1
if [[ "$MODE" == "prod" ]]; then
  echo "PROD mode"
  export FIZZ_CONFIG_DIR=$HOME/s/fizz/prod
  export DISCORD_TOKEN=`sh prod/SECRETS`
elif [[ "$MODE" == "staging" ]]; then
  echo "STAGING mode"
  export FIZZ_CONFIG_DIR=$HOME/s/fizz/staging
  export DISCORD_TOKEN=`sh staging/SECRETS`
else
  echo "Specify prod or staging mode."
  exit 1
fi
	 
cargo run
