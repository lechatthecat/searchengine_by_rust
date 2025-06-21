![alt text](https://github.com/lechatthecat/searchengine_by_rust/blob/main/img.png)

## How to use this search engine
At first, change .env for docker-compose and and .env for the search engine app.
```
(root directory)/.env
(root directory)/search/.env
```

Like this:
```
# URL
# REDIS_URL=redis:6379
# ELASTICSEARCH_URL=elasticsearch:9200
REDIS_URL=localhost:6379
ELASTICSEARCH_URL=localhost:9200

# Redis
REDIS_PASSWORD=test

# Elasticsearch
ELASTIC_PASSWORD=test
ELASTIC_USERNAME=elastic
```

please start redis & elasticsearch container by
```sh
(go to the project root directory)
$ docker compose -f docker-compose-dev.yml up --build
```

Don't close the terminal.

Keep it open and open a new terminal and please run commands below on the new terminal.

After the elasticsearch is started, you can send get-request (by using Bruno) to the following endpoint:
```
http://elastic:test@localhost:9200
```

Then make a put-request to the following endpoint:
```
http://elastic:test@localhost:9200/web_pages
```

with this body to make an index:
```json
{
  "settings": {
    "number_of_shards": 1,
    "number_of_replicas": 1
  },
  "mappings": {
    "properties": {
      "url": { "type": "keyword", "index": false },
      "title": { "type": "text", "analyzer" : "english" },
      "content": { "type": "text", "analyzer" : "english" },
      "page_rank": { "type": "float" },
      "inbound_links": { "type": "keyword" },  // Can also be nested if detailed info is needed
      "outbound_links": { "type": "keyword" },
      "page_updated_at": { "type": "date" },
      "page_created_at": { "type": "date" }
    }
  }
}
```

Now you can add document with post-request:
```
http://elastic:test@localhost:9200/web_pages/_doc/1
```

with this body
```json
{
  "url": "http://example.com/page1",
  "title": "Example Page 1",
  "content": "This is an example page for demonstrating PageRank.",
  "page_rank": 0.25,
  "inbound_links": ["http://example.com/page2", "http://example.com/page3"],
  "outbound_links": ["http://example.com/page4", "http://example.com/page5"]
}
```

Now you can run the rest api server and access the api:
```
$ cd search
$ cargo run

Now access from your browser:
http://localhost:8000/api/hello

(stop the server by ctrl+c then go back to the root by "cd ..")
```

Now change the .env:
```
# URL
REDIS_URL=redis:6379
ELASTICSEARCH_URL=elasticsearch:9200

# Redis
REDIS_PASSWORD=test

# Elasticsearch
ELASTIC_PASSWORD=test
ELASTIC_USERNAME=elastic
```

Go to the search folder again and build it.
```sh
$ cd search
$ cargo build --release
```

Then go to the frontend folder:
```sh
$ cd frontend
$ npm install
```

Now stop the containers started from docker-compose-dev.yml (by ctrl + c or docker compose down)!

Now please run this command:
```sh
(go to the project root directory)
$ docker compose -f docker-compose.yml up --build
```

Now the app should be running on http://localhost

## Other commands

Flush redis cache
```sh
$ docker exec -it redis redis-cli -a "$REDIS_PASSWORD" FLUSHALL
```

## Delete containers
Get inside the container
```sh
$ docker exec -it redis sh
```

If anything unusual happends, the logs are:
```shell
$ docker logs --tail 50 --follow --timestamps searchengine
$ docker logs --tail 50 --follow --timestamps redis
```

```shell
$ docker ps -a
```

To stop docker containers:

```shell
$ docker compose down 
```

Check volume
```shell
$ docker volume ls
$ docker volume inspect
```


To delete everything of docker. 

DON'T DO THIS IF YOU HAVE ANY CONTAINER/IMAGE YOU DON'T WANT TO DELETE.

THIS WILL DELETE EVERY IMAGE OF YOUR PC.

```shell
$ docker stop $(docker ps -aq)
# if you really want to delete all of docker data
$ docker compose down -v --rmi all --remove-orphans
$ docker rmi $(docker images -q) -f
$ docker volume rm $(docker volume ls -q)
$ docker system prune --force --volumes --all
```

Delete volumn
```shell
$ docker volume rm programmer_search_engine_db-store
```

