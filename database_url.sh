#!/usr/bin/env bash
export DATABASE_URL=mysql://kazuha@localhost/dojin-web

echo "DATABASE_URL=mysql://kazuha@localhost/dojin-web\nをexport"

echo "tableマクロを使って、「posts」と「user」を作成。"
read yes
echo "作成したなら、「1」を押してエンター"
if ${yes} -eq 1; then
    diesel setup
    diesel migration run
else
    echo "終わります"
fi

