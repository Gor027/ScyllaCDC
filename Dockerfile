FROM scylladb/scylla:latest

COPY resources/inserts.cql /docker-entrypoint-initdb.d/inserts.cql

COPY resources/cql_exec.sh /cql_exec.sh

ENTRYPOINT ["/cql_exec.sh"]
