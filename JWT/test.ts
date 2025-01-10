'use strict';

import jwt from "jsonwebtoken";
import * as fs from "node:fs";
import * as readline from "node:readline"; // 修正: "node:" プレフィックスを追加

// Create a readable stream
const rs = fs.createReadStream('./jwts.rand.txt');

// インターフェースの設定
const rl = readline.createInterface({
    input: rs, // 読み込みたいストリームの設定
});

// JWT secret key
const secret = 'ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC6eLDuYM4TKIKRvc5MSCPGiMSi7PYmbPMAkX6QbNc3PZtlvDjIL9ZYVsrVgw7FIvzpwzouqTV6K401AcRh7j24AXxNaH3OeC4uyx8u1u0mfxUB6DB6FjfjZsD46uqQvj6/GwZGLPkZ8Gyduqbi776Pb9LuBEyZo6wIGjQsHbELJmou4e2SeBEU6yF6MiFQ+DGK2xl6vikjslYzwXSCj7pD2hoVAc5nS5wjU5cf6rerDBcYmvjkN7qvBM+JkSUoWRjLbbyqoJJIHRQHQZay6HFOc88wCY+KHwnPg7+QWTNMpQgFDBQ0Rran1Mm/LH6HK7f0mT8Dl99zCBI/6BMKyVA5';

// 1行ずつ読み込む設定
rl.on('line', (token) => {
    try {
        const decoded = jwt.verify(token, secret);
        console.log('Decoded token:', decoded);
    } catch (err) {
    }
});
