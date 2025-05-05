INSERT INTO users (id, login_name, password_hash, role, created_at)
VALUES (
  gen_random_uuid(),
  'admin',
  '$argon2id$v=19$m=19456,t=2,p=1$vDfBLr+kb5ebIzMyXLP8VQ$RaLgexbHNV1uuBpYjDPSjqpyWYSeWmpZOJH7cdRyq00',
  'admin',
  CURRENT_TIMESTAMP
);
