# INE5429 - Números Primos

## Introdução

Este projeto foi desenvolvido como parte da disciplina de Segurança em Computação do Curso de Ciências da Computação na Universidade Federal de Santa Catarina do semestre 2021/1.

Essa pequena aplicação permite a geração de números aleatórios por meio de [MLCG](https://en.wikipedia.org/wiki/Lehmer_random_number_generator) e [LFMG](https://en.wikipedia.org/wiki/Lagged_Fibonacci_generator) e a pesquisa de nḿeros primos de diferentes tamanhos por meio de testes de [Miller-Rabin](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test) e [Fermat](https://en.wikipedia.org/wiki/Fermat_primality_test). Não espere ver as melhores práticas de programação e não se surpreenda com algum erro inesperado mas tentei resolver as principais possíveis causas de falha de execução, manter o código organizado e com seus métodos documentados.

## Requisitos

Para compilar o projeto você precisará da [toolchain de Rust](https://doc.rust-lang.org/book/ch01-00-getting-started.html) mas fora isso não é necessário nada para executar a aplicação além de estar usando uma distribuição de Linux (de preferência atualizada).

## Modo de uso

Após baixar ou clonar o repositório, basta executar **primetool** para utilizar o programa. Como teste você pode executar o seguinte comando:

> ./primetool -r --method m --number 5 --bits 32

Se você ver vários números aleatórios, logs de INFO com e um WARN no meio está tudo funcionando certinho.

O projeto possui algumas flags de execução, que você pode conferir com a flag **-h**, mas vale ressaltar que sempre você precisará selecionar uma, e apenas uma, flag de operação: **-r** para gerar números aleatórios ou **-p** para encontrar números primos.

## O que mais tem aqui?

Na pasta [graphs](https://github.com/dcroque/ine5429-primes/tree/main/graphs) existem alguns dados gerados pelo [flamegraph](https://github.com/flamegraph-rs/flamegraph) utilizados para a ánalise de gargalos da aplicação e na pasta [primes](https://github.com/dcroque/ine5429-primes/tree/main/primes) existem alguns arquivos com a geração de números primos de diferentes tamanhos, com o tempo de cada execução.