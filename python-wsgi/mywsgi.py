import urllib
import json


def application(environ, start_response):
    method = environ['REQUEST_METHOD']
    params = ""
    if method == 'GET':
        params = urllib.parse.parse_qsl(environ.get('QUERY_STRING'))
    elif method == 'POST':
        wsgi_input = environ["wsgi.input"]
        content_length = int(environ["CONTENT_LENGTH"])
        params = json.loads(wsgi_input.read(content_length))

    start_response('200 OK', [('Content-Type', 'text/html')])
    return [("Hello world:" + method + ",params=" + str(params) + "\n")
            .encode()]
