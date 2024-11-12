# Python FastAPI v. Rust actix-web

Tiny experiment to compare two web frameworks: Python FastAPI and Rust actix-web.

```bash
docker compose up --build -d
```

Send request to Rust actix-web server

```bash
curl http://localhost:8080/rust-adam
```

```text
"Hello rust-adam!
```

Send request to python FastAPI server:

```bash
curl http://localhost:8081/python-adam
```

```text
"Hello python-adam!
```

Send request to Rust rwf server:

```bash
curl http://localhost:8082/
```

```text
<h1>Hey Rwf!</h1>
```

Send request to Rust rwf server:

```bash
curl http://localhost:8083/
```

```text
Hello, Robyn world!
```

## Memory usage

```bash
docker stats --no-stream --format "table {{.Name}}\t{{.MemUsage}}"
```

```text
NAME                             MEM USAGE / LIMIT
python-v-rust-rust-rwf-1         4.121MiB / 15.54GiB
python-v-rust-python-robyn-1     50.09MiB / 15.54GiB
python-v-rust-python-fastapi-1   46.66MiB / 15.54GiB
python-v-rust-rust-actix-web-1   1.078MiB / 15.54GiB
```
