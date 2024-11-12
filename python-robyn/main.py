from robyn import Robyn

app = Robyn(__file__)

@app.get("/")
async def h(request):
    return "Hello, Robyn world!"

app.start(host="0.0.0.0", port=8083)
