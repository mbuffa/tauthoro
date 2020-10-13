# To tag the build with short SHA.
CURRENT = $(shell git rev-parse --short HEAD)

# [0]: I don't want to overwrite dependencies images with concurrent CI jobs,
# so I get the checksum of my manifest and use it as a tag...
BASEIMG = tauthoro/deps
BASETAG = $(shell sha1sum Cargo.lock | cut -d ' ' -f 1)
BASEIMGTAG = $(BASEIMG):$(BASETAG)

deps:
	docker build -t $(BASEIMGTAG) -t $(BASEIMG):latest -f Dockerfile.deps .

build:
	docker build -t tauthoro:$(CURRENT) --build-arg BASEIMGTAG="$(BASEIMGTAG)"  .

start:
	# FIXME: [0]... but it makes using docker-compose cumbersome.
	BASEIMGTAG=$(BASEIMGTAG) docker-compose up --build tauthoro

dump:
	docker-compose exec -- db pg_dump -h localhost -p 5432 -U tauthoro -d makks_authentication > docker/initdb.d/database_dump.sample.sql
