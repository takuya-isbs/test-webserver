import urllib
import json


def application(environ, start_response):
    method = environ['REQUEST_METHOD']
    path = environ.get("PATH_INFO", "")
    params = ""
    if method == 'GET':
        params = urllib.parse.parse_qs(environ.get('QUERY_STRING'))
    elif method == 'POST':
        wsgi_input = environ["wsgi.input"]
        content_length = int(environ["CONTENT_LENGTH"])
        params = json.loads(wsgi_input.read(content_length))

    start_response('200 OK', [('Content-Type', 'text/html')])
    # env_all = str(environ)
    # return [("path=" + path + ",METHOD="
    #          + method + ",params=" + str(params) + ",env=" + env_all + "\n")
    #         .encode()]
    return [("path=" + path + ",METHOD="
             + method + ",params=" + str(params) + "\n")
            .encode()]
