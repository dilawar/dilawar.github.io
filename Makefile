all :
	hugo mod get -u
	hugo server --minify --bind 0.0.0.0 --port 8000
