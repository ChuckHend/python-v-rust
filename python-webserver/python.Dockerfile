FROM python:3.13.0-bookworm

WORKDIR /app

COPY . .

RUN pip install -r requirements.txt
