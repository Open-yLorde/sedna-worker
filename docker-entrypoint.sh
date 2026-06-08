#!/bin/sh
set -e
# Apenas a API neste contentor. O worker é o serviço `worker` no docker-compose (evita dois
# processos worker ao mesmo tempo). Para monólito local: `SYNTHOSIA_EMBEDDED_WORKER=1 docker run ...`
if [ "${EMBEDDED_WORKER:-}" = "1" ]; then
  /usr/local/bin/sedna-worker &
fi
exec /usr/local/bin/sedna-worker