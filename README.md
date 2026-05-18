## 📝 Notas importantes

Durante este projeto aprendi bastante sobre **JWT**.

O mais interessante foi entender como JWT funciona de forma **stateless**, ou seja, não depende de persistência no servidor para continuar válido.

Mesmo que esta API utilize apenas armazenamento em memória volátil, um token gerado continua válido normalmente até sua expiração (24h), desde que:

- a chave secreta continue a mesma
- o token não esteja expirado

Achei esse comportamento muito interessante, porque mostra como autenticação com JWT pode funcionar sem armazenar sessão ou estado no backend.

---

Também foi meu primeiro contato com **Argon2**.

Provavelmente esta ainda não é a forma mais apropriada de utilizá-lo, mas foi a implementação que consegui construir após bastante leitura e testes.

Como Rust já possui uma curva de aprendizado alta por conta de ownership, borrowing, traits, async e tipos explícitos, adicionar criptografia e autenticação tornou o projeto significativamente mais desafiador.

Mesmo assim, estou gostando bastante da experiência com Rust 🦀

---

Neste projeto tentei reunir praticamente tudo que aprendi até agora:

- Axum
- Tokio
- handlers
- middleware
- modularização
- shared state
- JWT
- autenticação
- hashing de senha

Tive bastante dificuldade por estar lidando com muitos conceitos novos ao mesmo tempo e precisei usar IA para debugging e apoio em alguns trechos.

---

Um problema importante que encontrei foi relacionado a concorrência assíncrona.

Inicialmente usei:

```rust
std::sync::Mutex
```

Após muitas horas debugando, reescrevendo código e lendo documentação, descobri que esse mutex não é apropriado para uso em contexto async.

Ele bloqueia a thread inteira e pode causar problemas quando utilizado com `async/await`.

A solução foi migrar para:

```rust
tokio::sync::Mutex
```

Esse mutex é projetado para ambiente assíncrono e permite que tasks aguardem o lock sem bloquear a thread do executor.

Esse foi um dos bugs mais educativos até agora, porque me forçou a entender melhor:

- concorrência
- runtime assíncrono
- diferenças entre std e Tokio

---

---

## 🌐 Rotas da API

Esta API implementa um fluxo simples de autenticação usando JWT.

Os usuários são armazenados em memória através de:

```rust
HashMap<String, User>
```

A chave é o username.

---

### GET /

Health check da aplicação.

Usado para verificar se servidor está online.

Resposta:

```json
{
  "message": "API rodando"
}
```

---

### POST /register

Cria um novo usuário.

Body:

```json
{
  "username": "usuario123",
  "password": "senha123"
}
```

Validações:

- username mínimo de 8 caracteres
- username não vazio
- senha validada por regras do `Password`

Fluxo interno:

```txt
request
→ valida dados
→ gera hash com Argon2
→ cria User
→ salva em HashMap
```

Resposta:

```json
{
  "status_code": 201,
  "message": "Usuário criado"
}
```

Possíveis erros:

```json
{
  "status_code": 400,
  "message": "usuario e/ou senha invalidos"
}
```

---

### POST /login

Autentica usuário e retorna JWT.

Body:

```json
{
  "username": "usuario123",
  "password": "senha123"
}
```

Fluxo:

```txt
request
→ busca usuário
→ verifica senha
→ gera token JWT
→ retorna token
```

Resposta:

```json
{
  "token": "jwt_token_aqui"
}
```

---

### GET /profile

Rota protegida.

Necessário header:

```txt
Authorization: Bearer seu_token
```

Fluxo:

```txt
request
→ auth middleware
→ valida token
→ handler profile
→ decode token
→ retorna username
```

Resposta:

```json
{
  "user_name": "usuario123"
}
```

---

## 🔐 Middleware de autenticação

O middleware intercepta requests antes dos handlers protegidos.

Arquivo:

```txt
middleware/auth_middleware.rs
```

Fluxo:

```txt
request
↓
extrai Authorization
↓
remove Bearer
↓
verify_token()
↓
next.run(request)
```

Se token inválido:

```json
Unauthorized
```

Status:

```txt
401 Unauthorized
```

Trecho central:

```rust
if verify_token(&token, &data.jwt_secret) {
    next.run(request).await
} else {
    unauthorized
}
```

---

## 🧠 Estrutura dos dados

### UserRequest

Representa entrada do usuário.

```rust
pub struct UserRequest {
    pub username: String,
    pub password: String
}
```

Usado em:

- register
- login

---

### User

Representa usuário persistido em memória.

```rust
pub struct User {
    pub user_name: String,
    password_hash: Password
}
```

A senha nunca é armazenada em texto puro.

---

### AppState

Estado compartilhado da aplicação.

```rust
pub struct AppState {
    pub users: Arc<Mutex<HashMap<String, User>>>,
    pub config: Config
}
```

Contém:

- usuários
- configuração JWT

---

## 🔄 Fluxo completo

```txt
register
↓
validação
↓
hash senha
↓
salva usuário
↓
login
↓
verifica senha
↓
gera token
↓
profile
↓
middleware valida token
↓
retorna usuário
```

---

## ⚠️ Observações técnicas

Este projeto usa:

```rust
tokio::sync::Mutex
```

e não:

```rust
std::sync::Mutex
```

porque a aplicação trabalha com async/await.

O mutex do Tokio evita bloquear a thread inteira durante espera por lock.

---
