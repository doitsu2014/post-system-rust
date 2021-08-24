# Bind yourself
var_ca_subject='/C=VN/ST=HCM/L=HCM/O=Doitsu Technology/OU=DevOps/CN=www.doitsu.tech/emailAddress=dev@doitsu.tech'
# Bind yourself
var_localhost_subject='/C=VN/ST=HCM/L=HCM/O=Doitsu Technology/OU=DevOps/CN=localhost/emailAddress=dev@doitsu.tech'

var_ca_passphrase='zaQ@1234'
var_localhost_passphrase='zaQ@1234'
# openssl \
#     req \
#     -nodes \
#     -x509 \
#     -newkey rsa:4096 \
#     -keyout doitsu.tech.key \
#     -out doitsu.tech.crt \
#     -subj "/C=VN/ST=HCM/L=HCM/O=Doitsu Technology/OU=DevOps/CN=localhost.doitsu.tech/emailAddress=dev@doitsu.tech"
# openssl pkcs12 -export -inkey doitsu.tech.key -in doitsu.tech.crt -out doitsu.tech.pfx

openssl genrsa -out CA.key -passout pass:${var_ca_passphrase} -des3 2048
openssl req -x509 -sha256 -new -nodes -days 3650 -key CA.key -passin pass:${var_ca_passphrase} -out CA.pem -subj "${var_ca_subject}"

mkdir -p localhost
cd localhost

cat > localhost.ext << EOF
authorityKeyIdentifier = keyid,issuer
basicConstraints = CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
IP.1 = 127.0.0.1
EOF

openssl genrsa -out localhost.key -passout pass:${var_localhost_passphrase} -des3 2048
openssl req -new -key localhost.key -passin pass:${var_localhost_passphrase} -out localhost.csr -subj "${var_localhost_subject}"

openssl x509 -req -in localhost.csr -CA ../CA.pem -CAkey ../CA.key -passin pass:${var_ca_passphrase} -CAcreateserial -days 3650 -sha256 -extfile localhost.ext -out localhost.crt
openssl rsa -in localhost.key -passin pass:${var_ca_passphrase} -out localhost.decrypted.key