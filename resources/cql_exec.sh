#!/bin/bash

for f in docker-entrypoint-initdb.d/*; do
  case "$f" in
  *.cql) echo "$0: running $f" &&
    until cqlsh -f "$f"; do
      echo >&2 "Unavailable: sleeping"
      sleep 10
    done &;;
  esac
  echo
done

exec /docker-entrypoint.py "$@"
