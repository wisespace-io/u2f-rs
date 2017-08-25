mkdir private
cd private/

openssl req -x509 -newkey rsa:4096 -nodes -sha256 -days 3650 -keyout key.pem -out cert.pem