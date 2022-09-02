.PHONY: all
all: build

clean:
	cargo clean

build:
	cargo sqlx prepare --database-url sqlite:database.sqlite

rebuild: rebuild_db build

full_rebuild: clean rebuild

rebuild_db:
	rm database.*
	sqlx database create --database-url sqlite:database.sqlite
	sqlx migrate run --database-url sqlite:database.sqlite