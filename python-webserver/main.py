from typing import Union

from fastapi import FastAPI

app = FastAPI()


@app.get("/{name}")
def read_item(name: str):
    return f"Hello {name}!"
