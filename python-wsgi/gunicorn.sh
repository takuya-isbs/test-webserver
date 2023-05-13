#!/bin/sh

GUNICORN_DIR=/usr/local
USERNAME=root  #TODO
GROUPNAME=root
LOGLEVEL=
THREAD_NUM=2
WORKER_NUM=2
BIND_ADDR_PORT=0.0.0.0:8000
BASEDIR=/SRC
TIMEOUT=180

#RELOAD="--reload"
RELOAD=""

exec "${GUNICORN_DIR}/bin/gunicorn" \
     --user $USERNAME \
     --group $GROUPNAME \
     --log-level "$LOGLEVEL" \
     --error-logfile - \
     --capture-output \
     --threads "$THREAD_NUM" \
     --workers "$WORKER_NUM" \
     --bind "$BIND_ADDR_PORT" \
     --chdir "$BASEDIR" \
     --timeout $TIMEOUT \
     $RELOAD \
     mywsgi:application
