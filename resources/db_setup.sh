#!/usr/bin/sh
# Utility script to generate insert statements

start_t=0
v_counter=0
s_counter=0

while :
do
  echo "INSERT INTO t (pk, t, v, s) VALUES (0, $start_t, 'val$v_counter', 'static$s_counter');" >> inserts.cql
  start_t=$((start_t+1))
  v_counter=$((v_counter+1))
  s_counter=$((s_counter+1))
  sleep 1
done
