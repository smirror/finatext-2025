'use strict';

const jwt = require('jsonwebtoken');
const fs = require('fs');
const readline = require('readline');

// Create a readable stream
const rs = fs.createReadStream('./jwts.rand.txt');

// インターフェースの設定
const rl = readline.createInterface({
    input: rs, // 読み込みたいストリームの設定
});

// JWT secret key
const publicKey = fs.readFileSync('./jwts.sec.key', 'utf-8');

// 1行ずつ読み込む設定
let counter = 0;
rl.on('line', (token) => {
    try {
        const decoded = jwt.verify(token, publicKey, {algorithms: ['RS256']});
        console.log('JWTは公開鍵で正しく署名されています。デコード結果:', decoded);
    } catch (err) {
        counter++;
    }
});

rl.on('close', () => {
    console.log('JWTの検証結果: 無効な署名が', counter, '個あります。');
});