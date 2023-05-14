#!/bin/sh

set -eux

cd /SRC
pip install --upgrade pip
pip install -r requirements.txt

flake8 mywsgi.py test_mywsgi.py
pytest test_mywsgi.py
