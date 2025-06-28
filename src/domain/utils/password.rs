pub fn hash_password(password: &str) -> String {
    // 簡易的なハッシュ関数（実際のプロジェクトではbcryptなどを使用）
    format!("hashed_{}", password)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}
