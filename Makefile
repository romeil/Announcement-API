install:
	cargo add actix
	cargo add actix-files
	cargo add actix-session --features="redis-rs-session"
	cargo add actix-web --features=openssl
	cargo add actix-web-httpauth
	cargo add bcrypt
	cargo add chrono
	cargo add dotenv
	cargo add env_logger
	cargo add futures_util
	cargo add hex
	cargo add lazy_static
	cargo add openssl
	cargo add pasetors
	cargo add pgp
	cargo add serde --features=derive
	cargo add rand
	cargo add regex
	cargo add ring
	cargo add serde --features=derive
	cargo add serde_json
	cargo add sqlx --features="runtime-async-std-native-tls postgres uuid"
	cargo add tera
	cargo add uuid --features ="v4 serde"
	cargo add validators --features=email