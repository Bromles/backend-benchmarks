from asgiref.wsgi import WsgiToAsgi
from flask import Flask
import uvicorn

app = Flask(__name__)
asgi_app = WsgiToAsgi(app)


@app.route("/")
def hello_world():
    return "Hello, World!"


if __name__ == "__main__":
    uvicorn.run(asgi_app, host="0.0.0.0", port=3000)
