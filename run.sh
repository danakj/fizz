#!/bin/sh

MODE=$1
if [[ "$MODE" != "staging" ]]; then
  echo "PROD mode"
  export FIZZ_CONFIG_DIR=$HOME/s/fizz/prod
  export DISCORD_TOKEN=`sh prod/SECRETS`
else
  echo "STAGING mode"
  export FIZZ_CONFIG_DIR=$HOME/s/fizz/staging
  export DISCORD_TOKEN=`sh staging/SECRETS`
fi
	 
cargo run
