# Python FastAPI v. Rust actix-web

Tiny experiment to compare two web frameworks: Python FastAPI and Rust actix-web.

```bash
docker compose up --build -d
```

Send request to rust server

```bash
curl http://localhost:8080/rust-adam
"Hello rust-adam!
```

Send request to python server:

```bash
curl http://localhost:8081/python-adam
"Hello python-adam!
```

## Memory usage

When idle, the python server uses about 40x more memory.

```bash
docker stats
```

```text
CONTAINER ID   NAME                               CPU %     MEM USAGE / LIMIT     MEM %     NET I/O           BLOCK I/O         PIDS
97b48369c95c   python-v-rust-rust-webserver-1     0.04%     1.191MiB / 13.64GiB   0.01%     1.66kB / 489B     0B / 0B           3
de7a30f9fc65   python-v-rust-python-webserver-1   0.13%     44.49MiB / 13.64GiB   0.32%     1.66kB / 501B     0B / 0B           7
```
