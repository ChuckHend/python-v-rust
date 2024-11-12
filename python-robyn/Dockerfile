FROM python:3.12-bookworm

RUN apt-get update && apt-get install -y \
    curl build-essential make 

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY . .

RUN pip install -r requirements.txt
