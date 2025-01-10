#!/bin/bash

CA_FILE="ca.pem"    # 提供されたCA証明書
CERTS_DIR="certs"   # 証明書ディレクトリ

for cert in $CERTS_DIR/*.pem; do
    # OpenSSLで証明書を検証
    echo "証明書の検証: $cert"
    if openssl verify -CAfile "$CA_FILE" "$cert" > /dev/null 2>&1; then
        echo "一致する証明書のファイル名: $cert"
        break
    fi
done
