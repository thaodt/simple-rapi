#!/bin/bash

export POSTGRES_PASSWORD=$(cat /run/secrets/db_pass)

exec "$@"