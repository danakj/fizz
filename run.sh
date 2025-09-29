#!/bin/sh

MODE=$1
if [[ "$MODE" != "staging" ]]; then
  echo "PROD mode"
  export FIZZ_CONFIG_DIR=$HOME/s/fizz/prod
else
  echo "STAGING mode"
  export FIZZ_CONFIG_DIR=$HOME/s/fizz/staging
fi
	 
export DISCORD_TOKEN=`sh SECRETS`
cargo run
