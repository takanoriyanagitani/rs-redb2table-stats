#!/bin/bash

set -u

wsm='./redb2table-stats.wasm'

iredb='example.redb'
hostd="${PWD}"
guestd='/guest.d'
redb4guest="${guestd}/${iredb}"

run_wasi(){
  wasmtime \
    run \
    --dir "${hostd}::${guestd}" \
    --env ENV_REDB_FILENAME="${redb4guest}" \
    "${wsm}"
}

test -f "${iredb}" || exec env dbname="${iredb}" sh -c '
  echo redb file ${dbname} not found.
  echo you need to create the db to run this example.
  exit 1
'

run_wasi
