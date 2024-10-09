#FROM python:3.11-bullseye
FROM python:3.13-bookworm

RUN python3 -m pip install gunicorn \
    && mkdir /SRC
COPY ./gunicorn.sh /SRC/
COPY ./mywsgi.py /SRC/
COPY ./test_mywsgi.py /SRC/
COPY ./test.sh /SRC/
COPY ./requirements.txt /SRC/

CMD ["/bin/sh", "/SRC/gunicorn.sh"]
