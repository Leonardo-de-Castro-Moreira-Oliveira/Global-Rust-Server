# Global Rust Server

## Descrição

O **Global Rust Server** é um servidor construído em Rust que oferece uma API para gerenciar usuários. Este projeto é uma demonstração de como usar Rust e a biblioteca Actix-web para criar uma aplicação web robusta e escalável. O objetivo principal é fornecer uma plataforma leve e eficiente para gerenciar informações de usuários em uma base de dados PostgreSQL.

## Objetivos

1. **Gerenciamento de Usuários:** Permitir a criação, leitura, atualização e exclusão de usuários através de uma API RESTful.
2. **Desempenho:** Aproveitar as características de alto desempenho do Rust, criando um servidor que pode lidar com múltiplas requisições simultâneas.
3. **Aprendizado:** Explorar o ecossistema Rust e suas bibliotecas, como `sqlx` para interações com o banco de dados e `actix-web` para a construção do servidor.

## Tecnologias Utilizadas

- **Rust:** A linguagem de programação escolhida para o desenvolvimento do servidor devido à sua eficiência, segurança de memória e performance.
- **Actix-web:** Um framework web para Rust que é rápido e leve, permitindo a criação de APIs RESTful de maneira eficiente.
- **SQLx:** Uma biblioteca assíncrona de interação com bancos de dados que facilita a comunicação com o PostgreSQL.
- **PostgreSQL:** O sistema de gerenciamento de banco de dados relacional utilizado para armazenar informações dos usuários.

## Escolhas de Design

1. **Estrutura Assíncrona:** O uso de funcionalidades assíncronas em Rust permite que o servidor gerencie várias requisições ao mesmo tempo, resultando em melhor desempenho e menor latência.
2. **Separação de Camadas:** A aplicação foi projetada com uma clara separação de responsabilidades, com camadas distintas para o gerenciamento de rotas, lógica de negócios e acesso a dados.
3. **Modelo de Dados:** A escolha do `UserSchema` para mapear a estrutura dos dados no banco de dados garante que o acesso aos dados seja feito de maneira segura e eficiente.

## Como Começar

### Pré-requisitos

Antes de começar, você precisará ter o seguinte instalado:

- Rust (com `cargo`)
- PostgreSQL

### Instalação

1. Clone este repositório:

   ```bash
   git clone https://github.com/Leonardo-de-Castro-Moreira-Oliveira/Global-Rust-Server.git
   cd Global-Rust-Server

2. Configure seu banco de dados PostgreSQL e crie a tabela rust_user:
   
   ```sql
   CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

   CREATE TABLE
       IF NOT EXISTS rust_user (
           id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
           name VARCHAR(255) NOT NULL UNIQUE,
           password VARCHAR(255) NOT NULL
       );


3. Compile e execute o servidor:
   
   ```bash
    cargo run
   
