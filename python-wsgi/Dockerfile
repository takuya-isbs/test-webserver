FROM python:3.11-bullseye

RUN python3 -m pip install gunicorn \
    && mkdir /SRC
COPY ./gunicorn.sh /SRC/
COPY ./testwsgi.py /SRC/

CMD ["/bin/sh", "/SRC/gunicorn.sh"]
