migrate:
	cd libs/data/db && surrealdb-migrations apply \
		--address "ws://${DB_ADDR}" \
		--username "${DB_USER}" \
		--password "${DB_PASSWORD}" \
		--ns athena \
		--db athena
