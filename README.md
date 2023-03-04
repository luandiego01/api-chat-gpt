# Projeto para uso da API do ChatGPT pelo RUST

##  Chave da API

* Você pode conseguir sua chave se cadastrando no site da OpenAI e acessando esse [link](https://platform.openai.com/account/api-keys)

Obs: Existe um custo para utilização tanto para a API do ChatGPT, quanto para as outras APIs da OpenAI

[Preços das APIs](https://openai.com/pricing)

Com a sua chave, basta substituir na variável `const API_KEY` no arquivo `src\main.rs`

## Uso da API

Optei por salvar as duas últimas conversas da, e uma mensagem inicial pro sistema, nesse caso, "Você é um assistente útil", porém podemos usar outras frases para adaptar o ChatGPT para nos auxiliar, por exemplo:

* "Você é um bot que só responde como se tivesse ensinando para uma criança"
* "Você é um bot que responde tudo com emojis"
* "Você é um bot que não sabe nenhuma resposta"

Também fixei a temperatura (aleatoriedade da resposta) em 0.8 e a quantidade máxima de tokens da resposta em 200, caso queiram saber mais dos inputs da API, basta acessar a [documentação](https://platform.openai.com/docs/api-reference/chat)

Todas essas features podem ser alteradas no começo do código no arquivo `src\main.rs`

* Por fim, após clonar esse repositório, basta ir na pasta dele e executar o comando:

```cargo run```

Caso ainda não tenha o Rust e o Cargo instalado, segue o site oficial para auxílio da instalação:

[Instalação](https://www.rust-lang.org/tools/install)