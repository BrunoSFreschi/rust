# Guia de estudo de Rust

Baseado no roadmap do arquivo `rust.pdf`.

## Objetivo

Aprender Rust de forma progressiva: primeiro a linguagem, depois organização de projetos, testes, concorrência, bibliotecas do ecossistema e projetos práticos.

## Como usar este guia

- Estude em ciclos curtos: leia, escreva código, rode, quebre e corrija.
- Para cada tópico, crie pequenos arquivos ou projetos com `cargo new`.
- Não avance rápido demais em ownership, borrowing e lifetimes. Esses conceitos sustentam quase todo o resto.
- Ao final de cada etapa, faça o projeto sugerido antes de seguir.

## 1. Introdução e ambiente

### O que aprender

- O que é Rust.
- Por que usar Rust.
- Instalação do Rust e Cargo.
- Uso do Rust Playground.
- Configuração de IDE/editor.

### Conceitos principais

Rust é uma linguagem de sistemas focada em segurança de memória, performance e concorrência. Ela evita muitos erros comuns sem depender de garbage collector.

Cargo é a ferramenta principal para criar projetos, compilar, rodar testes, baixar dependências e publicar pacotes.

### Comandos essenciais

```bash
rustc --version
cargo --version
cargo new hello_rust
cd hello_rust
cargo run
cargo build
cargo check
```

### Prática

Crie um projeto `hello_rust` que imprime seu nome, idade e uma frase usando variáveis.

## 2. Sintaxe, semântica e fundamentos

### O que aprender

- Variáveis.
- Mutabilidade.
- Constantes.
- Tipos de dados.
- Controle de fluxo.
- Funções.
- Métodos.
- Pattern matching.
- Desestruturação.

### Tópicos

#### Variáveis e constantes

Em Rust, variáveis são imutáveis por padrão.

```rust
let nome = "Bruno";
let mut idade = 30;
idade += 1;

const LIMITE: u32 = 100;
```

#### Tipos básicos

- Inteiros: `i32`, `u32`, `usize`.
- Ponto flutuante: `f32`, `f64`.
- Booleanos: `bool`.
- Caracteres: `char`.
- Strings: `String` e `&str`.
- Tuplas.
- Arrays.

#### Controle de fluxo

Estude:

- `if`, `else if`, `else`.
- `loop`.
- `while`.
- `for`.
- `match`.

#### Funções

```rust
fn soma(a: i32, b: i32) -> i32 {
    a + b
}
```

#### Pattern matching

```rust
let numero = 2;

match numero {
    1 => println!("um"),
    2 => println!("dois"),
    _ => println!("outro"),
}
```

### Prática

Crie uma calculadora simples com funções para somar, subtrair, multiplicar e dividir.

## 3. Constructs: structs, enums, traits e impl

### O que aprender

- `struct`.
- `enum`.
- `trait`.
- Blocos `impl`.

### Structs

Use structs para representar dados relacionados.

```rust
struct Usuario {
    nome: String,
    ativo: bool,
}

impl Usuario {
    fn novo(nome: String) -> Self {
        Self { nome, ativo: true }
    }
}
```

### Enums

Enums representam valores que podem assumir uma entre várias formas.

```rust
enum Status {
    Pendente,
    Concluido,
    Cancelado,
}
```

### Traits

Traits definem comportamento compartilhado.

```rust
trait Descrever {
    fn descrever(&self) -> String;
}
```

### Prática

Modele um sistema de tarefas:

- Uma `struct Tarefa`.
- Um `enum Status`.
- Um método para marcar tarefa como concluída.
- Um trait `Resumo` que retorna uma descrição da tarefa.

## 4. Estruturas de dados

### O que aprender

- `Array`.
- `Vec`.
- `HashMap`.
- `HashSet`.
- `LinkedList`.
- Pilha.
- Fila.
- `BinaryHeap`.
- `BTreeMap`.
- `BTreeSet`.

### Foco inicial

Priorize:

- `Vec<T>`.
- `HashMap<K, V>`.
- `HashSet<T>`.

Essas estruturas aparecem com muita frequência em programas reais.

### Exemplo com `HashMap`

```rust
use std::collections::HashMap;

fn main() {
    let mut notas = HashMap::new();
    notas.insert("Ana", 9);
    notas.insert("Joao", 8);

    for (aluno, nota) in notas {
        println!("{aluno}: {nota}");
    }
}
```

### Prática

Crie um contador de palavras:

- Receba uma frase.
- Separe as palavras.
- Conte quantas vezes cada palavra aparece usando `HashMap`.

## 5. Ownership, borrowing e memória

### O que aprender

- Regras de ownership.
- Segurança de memória.
- Borrowing.
- Referências.
- Slices.
- Diferença entre stack e heap.

### Regras de ownership

1. Cada valor em Rust tem um dono.
2. Só pode existir um dono por vez.
3. Quando o dono sai de escopo, o valor é descartado.

### Borrowing

Use referências quando quiser emprestar um valor sem transferir ownership.

```rust
fn tamanho(texto: &String) -> usize {
    texto.len()
}

fn main() {
    let nome = String::from("Rust");
    println!("{}", tamanho(&nome));
    println!("{}", nome);
}
```

### Referência mutável

```rust
fn adicionar(texto: &mut String) {
    texto.push_str(" language");
}
```

### Slices

```rust
let texto = String::from("aprendendo rust");
let parte = &texto[0..10];
```

### Prática

Faça funções que:

- Recebem `String` por valor.
- Recebem `&String`.
- Recebem `&str`.
- Recebem `&mut String`.

Compare o que compila e o que não compila.

## 6. Tratamento de erros

### O que aprender

- `Option<T>`.
- `Result<T, E>`.
- Operador `?`.
- Tipos de erro customizados.
- Traits de erro.

### Option

Use `Option` quando um valor pode existir ou não.

```rust
fn primeiro(v: Vec<i32>) -> Option<i32> {
    v.first().copied()
}
```

### Result

Use `Result` quando uma operação pode falhar.

```rust
use std::fs;

fn ler_arquivo(caminho: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(caminho)
}
```

### Operador `?`

```rust
fn ler_config() -> Result<String, std::io::Error> {
    let conteudo = std::fs::read_to_string("config.txt")?;
    Ok(conteudo)
}
```

### Prática

Crie um programa que lê um arquivo, conta suas linhas e trata erro caso o arquivo não exista.

## 7. Módulos, crates e Cargo

### O que aprender

- Organização de código.
- Módulos.
- Crates.
- Namespacing.
- Dependências no `Cargo.toml`.
- Publicação no crates.io.

### Estrutura comum

```text
meu_projeto/
  Cargo.toml
  src/
    main.rs
    lib.rs
    usuarios.rs
```

### Exemplo de módulo

```rust
mod usuarios;

fn main() {
    usuarios::criar();
}
```

### Dependências

```bash
cargo add serde
cargo add serde_json
```

Se `cargo add` não estiver disponível, edite o `Cargo.toml` manualmente.

### Prática

Separe o projeto de tarefas em módulos:

- `tarefa.rs`.
- `status.rs`.
- `main.rs`.

## 8. Testes

### O que aprender

- Testes unitários.
- Testes de integração.
- Mocks.
- Testes baseados em propriedades.

### Teste unitário

```rust
fn soma(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soma_dois_numeros() {
        assert_eq!(soma(2, 3), 5);
    }
}
```

### Comando

```bash
cargo test
```

### Prática

Adicione testes para a calculadora e para o sistema de tarefas.

## 9. Traits e generics

### O que aprender

- Definição e implementação de traits.
- Trait bounds.
- Associated types.
- Generics.
- Programação em nível de tipo.

### Generic simples

```rust
fn primeiro<T>(lista: &[T]) -> Option<&T> {
    lista.first()
}
```

### Trait bound

```rust
use std::fmt::Display;

fn imprimir<T: Display>(valor: T) {
    println!("{valor}");
}
```

### Prática

Crie uma função genérica que recebe uma lista e retorna o maior item. Depois imponha os trait bounds necessários.

## 10. Lifetimes e borrow checker

### O que aprender

- Lifetimes.
- Anotações explícitas.
- Regras de elisão.
- Covariância e contravariância.

### Ideia principal

Lifetimes dizem ao compilador por quanto tempo referências são válidas. Muitas vezes Rust infere isso automaticamente, mas em funções que retornam referências pode ser necessário declarar.

```rust
fn maior<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
```

### Prática

Crie funções que retornam referências e observe quando o compilador exige anotações de lifetime.

## 11. Concorrência e paralelismo

### O que aprender

- Threads.
- Channels.
- Message passing.
- Operações atômicas.
- Memory barriers.
- Futures.
- `async` e `await`.

### Tipos importantes

- `std::thread`.
- `std::sync::mpsc`.
- `Arc<T>`.
- `Mutex<T>`.
- `RwLock<T>`.

### Exemplo com thread

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("executando em outra thread");
    });

    handle.join().unwrap();
}
```

### Prática

Crie um programa que divide uma lista de números entre várias threads e soma os resultados parciais.

## 12. Programação assíncrona

### O que aprender

- `async`.
- `await`.
- Runtimes assíncronos.
- Tokio.
- async-std.
- smol.

### Foco recomendado

Comece com Tokio, pois é muito usado no ecossistema Rust.

### Bibliotecas úteis

- `tokio`.
- `reqwest`.
- `hyper`.
- `quinn`.

### Prática

Crie um programa que faz requisições HTTP para várias URLs em paralelo e imprime o status de cada resposta.

## 13. Macros e metaprogramação

### O que aprender

- Macros declarativas com `macro_rules!`.
- Macros procedurais.
- Custom derive.
- DSLs.

### Comece por `macro_rules!`

```rust
macro_rules! ola {
    () => {
        println!("Ola, Rust!");
    };
}

fn main() {
    ola!();
}
```

### Prática

Crie uma macro simples para imprimir mensagens de log com prefixo.

## 14. Ecossistema e bibliotecas

### Web development

Bibliotecas citadas no roadmap:

- Axum.
- Actix.
- Rocket.
- Leptos.
- Loco.

Recomendação do roadmap: Axum.

### Serialização

- `serde`.
- `json-rust`.
- `toml-rust`.

Priorize `serde` com `serde_json`.

### Banco de dados e ORM

- Diesel.
- sqlx.
- rusqlite.

Comece por `sqlx` se quiser trabalhar com async e APIs web. Use `rusqlite` para projetos locais com SQLite.

### CLI

- `clap`.
- `structopt`.
- `termion`.

Priorize `clap`, pois `structopt` foi absorvido pelo ecossistema do `clap`.

### Criptografia

- `ring`.
- `rust-crypto`.
- `sodiumoxide`.

### GUI

- Tauri.
- gtk-rs.
- relm.

### Game development

- Bevy.
- Fyrox.
- ggez.
- macroquad.
- wgpu-rs.

### Embedded e sistemas

- embedded-hal.
- rppal.
- nrf-hal.

### WebAssembly

- wasm-pack.
- wasm-bindgen.
- wasmer.

### Performance e debugging

- Criterion.rs.
- `rustdoc`.
- `rust-gdb`.
- `rust-lldb`.

## 15. Projetos práticos em ordem

### Projeto 1: Calculadora CLI

Conceitos:

- Funções.
- Match.
- Entrada e saída.
- Tratamento básico de erro.

### Projeto 2: Contador de palavras

Conceitos:

- Strings.
- Slices.
- `HashMap`.
- Iteradores.

### Projeto 3: Gerenciador de tarefas

Conceitos:

- Structs.
- Enums.
- Traits.
- Módulos.
- Testes.

### Projeto 4: Leitor de arquivos

Conceitos:

- `Result`.
- Operador `?`.
- Erros de IO.
- Organização de código.

### Projeto 5: CLI com `clap`

Conceitos:

- Dependências.
- Cargo.
- Argumentos de linha de comando.
- Separação entre binário e biblioteca.

### Projeto 6: API REST com Axum

Conceitos:

- Async.
- Tokio.
- Rotas.
- JSON com Serde.
- Estado compartilhado.
- Testes de handlers.

### Projeto 7: API com banco de dados

Conceitos:

- sqlx.
- SQLite ou Postgres.
- Migrações.
- Repositórios.
- Erros customizados.

### Projeto 8: Benchmark e profiling

Conceitos:

- Criterion.rs.
- Otimização.
- Comparação entre estruturas de dados.

## 16. Ordem recomendada de estudo

1. Instalação, Cargo e primeiro programa.
2. Variáveis, tipos, controle de fluxo e funções.
3. Structs, enums, impl e traits.
4. Ownership, borrowing, referências e slices.
5. `Option`, `Result` e operador `?`.
6. Coleções: `Vec`, `HashMap`, `HashSet`.
7. Módulos, crates e dependências.
8. Testes.
9. Generics, trait bounds e associated types.
10. Lifetimes.
11. Concorrência com threads, channels, `Arc` e `Mutex`.
12. Async com Tokio.
13. Web com Axum.
14. Banco de dados com sqlx.
15. Macros.
16. Performance, profiling e documentação.

## 17. Rotina semanal sugerida

### Semana 1

- Instalar Rust.
- Aprender Cargo.
- Estudar variáveis, tipos, funções e controle de fluxo.
- Fazer a calculadora CLI.

### Semana 2

- Estudar structs, enums, impl e traits.
- Estudar `Vec`, `HashMap` e `HashSet`.
- Fazer o contador de palavras.

### Semana 3

- Estudar ownership, borrowing e slices.
- Refazer exercícios antigos corrigindo problemas de ownership.
- Fazer o gerenciador de tarefas.

### Semana 4

- Estudar `Option`, `Result`, `?` e erros customizados.
- Estudar módulos e organização com Cargo.
- Adicionar testes aos projetos.

### Semana 5

- Estudar generics, trait bounds e lifetimes.
- Melhorar os projetos anteriores usando código genérico onde fizer sentido.

### Semana 6

- Estudar threads, channels, `Arc`, `Mutex` e `RwLock`.
- Fazer projeto de soma paralela ou processamento paralelo de arquivos.

### Semana 7

- Estudar async, await e Tokio.
- Fazer cliente HTTP assíncrono com `reqwest`.

### Semana 8

- Estudar Axum, Serde e sqlx.
- Fazer uma API REST simples com banco de dados.

## 18. Checklist de progresso

- [ ] Instalei Rust e Cargo.
- [ ] Criei e rodei um projeto com `cargo run`.
- [ ] Entendi variáveis imutáveis e `mut`.
- [ ] Usei tipos básicos.
- [ ] Escrevi funções com retorno.
- [ ] Usei `if`, `loop`, `while`, `for` e `match`.
- [ ] Criei structs.
- [ ] Criei enums.
- [ ] Implementei métodos com `impl`.
- [ ] Criei e implementei traits.
- [ ] Usei `Vec`.
- [ ] Usei `HashMap`.
- [ ] Entendi as regras de ownership.
- [ ] Usei referências imutáveis.
- [ ] Usei referências mutáveis.
- [ ] Usei slices.
- [ ] Usei `Option`.
- [ ] Usei `Result`.
- [ ] Usei o operador `?`.
- [ ] Organizei código em módulos.
- [ ] Adicionei dependências no `Cargo.toml`.
- [ ] Escrevi testes unitários.
- [ ] Escrevi testes de integração.
- [ ] Usei generics.
- [ ] Usei trait bounds.
- [ ] Entendi lifetimes básicos.
- [ ] Criei threads.
- [ ] Usei channels.
- [ ] Usei `Arc` e `Mutex`.
- [ ] Escrevi código async com Tokio.
- [ ] Criei uma API com Axum.
- [ ] Serializei JSON com Serde.
- [ ] Usei banco de dados com sqlx ou rusqlite.
- [ ] Fiz benchmark com Criterion.rs.
- [ ] Gerei documentação com `cargo doc`.

## 19. Comandos úteis

```bash
cargo new nome_do_projeto
cargo run
cargo build
cargo build --release
cargo check
cargo test
cargo doc --open
cargo fmt
cargo clippy
```

## 20. Próximo passo

Comece pelo projeto `hello_rust`, depois faça a calculadora CLI. Enquanto escreve código, mantenha uma lista dos erros do compilador que você encontrou e o que aprendeu com cada um. Em Rust, entender as mensagens do compilador é parte central do aprendizado.
