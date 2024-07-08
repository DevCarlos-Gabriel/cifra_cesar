# Projeto de Criptografia e Descriptografia 🔐

Este projeto é uma implementação simples de um programa de criptografia e descriptografia em Rust, usando um alfabeto padrão e deslocamento de caracteres. O programa pode criptografar e descriptografar mensagens de texto com base em um número de deslocamento especificado pelo usuário.

## Funcionalidades ✨

- **Criptografia**: Transforma uma mensagem de texto em uma mensagem cifrada usando um deslocamento especificado no alfabeto.
- **Descriptografia**: Reverte a mensagem cifrada de volta para a mensagem original usando o mesmo deslocamento.

## Como Usar 🚀

1. **Clone o repositório:**
    ```sh
    git clone https://github.com/DevCarlos-Gabriel/cifra_cesar.git cifra_cesar
    cd cifra_cesar
    ```

2. **Compile o programa:**
    ```sh
    cargo build
    ```

3. **Execute o programa:**
    ```sh
    cargo run
    ```
## Exemplos de Uso 📋

Ao iniciar o programa, você verá o seguinte prompt:

```sh
Informe uma das seguintes opções:
Criptografar
Descriptografar
```
### Criptografar 🔒

1. Digite `Criptografar` e pressione Enter.
2. Insira a mensagem que você deseja criptografar.
3. Insira o número de deslocamento (quantas letras serão deslocadas no alfabeto).

Exemplo de entrada:

```sh
Criptografar
Olá mundo
3
```

Saída esperada:

```sh
A mensagem criptografada fica assim: Roá Pxqgr
```
### Descriptografar 🔓

1. Digite `Descriptografar` e pressione Enter.
2. Insira a mensagem que você deseja descriptografar.
3. Insira o número de deslocamento usado para criptografar a mensagem.

Exemplo de entrada:

```sh
Descriptografar
Roá Pxqgr
3
```
Saída esperada:

```sh
A mensagem descriptografada fica assim: Olá Mundo
```
## Estrutura do Código 📂

O código é estruturado da seguinte forma:

1. **Constantes**: Definição dos alfabetos maiúsculos e minúsculos.
2. **Função principal (`main`)**: Responsável por interagir com o usuário, capturar a entrada e chamar as funções de criptografia e descriptografia conforme necessário.
3. **Função `has_accent`**: Verifica se um caractere possui acento.
4. **Função `encrypt`**: Realiza a criptografia da mensagem.
5. **Função `decrypt`**: Realiza a descriptografia da mensagem.

## Dependências 📦

Este projeto usa a crate `unicode_normalization` para lidar com caracteres acentuados. Adicione a seguinte dependência ao seu `Cargo.toml`:

```toml
[dependencies]
unicode-normalization = "0.1.22"
```
## Considerações Finais 📝

Contribuições são bem-vindas! Sinta-se à vontade para abrir um PR ou relatar problemas.

## Licença 📄

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE]() para mais detalhes.
