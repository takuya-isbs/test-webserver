from webtest import TestApp
import mywsgi
import urllib.parse


class Test_wsgi_app(object):
    def test_get(self):
        app = TestApp(mywsgi.application)
        params = {'name': ['Alice', 'Bob']}
        encstr = urllib.parse.urlencode(params, doseq=True)
        resp = app.get(f'/abc?{encstr}')
        assert resp.status_code == 200
        assert resp.content_type == 'text/html'
        s = resp.body.decode()
        ex = "path=/abc,METHOD=GET,params={'name': ['Alice', 'Bob']}"
        assert s.startswith(ex)

    def test_post(self):
        app = TestApp(mywsgi.application)
        resp = app.post_json('/def',
                             {"id": 123, "flag": True})
        assert resp.status_code == 200
        assert resp.content_type == 'text/html'
        s = resp.body.decode()
        print(s)
        ex = "path=/def,METHOD=POST,params={'id': 123, 'flag': True}"
        assert s.startswith(ex)
