# Como ler esses gráficos

Os gráficos estão nomeados para indicar os parâmetros de execução na forma de M_N_T_S, sendo:

- M: Método utilizado
  - mlcg: Multiplicative Linear Congruential Generator
  - lf: Lagged Fibonacci Multiplicative Generator
  - millerrabin: Teste de Miller-Rabin
  - fermat: Teste de Fermat
- N: Número de gerações
- T: Tamanho em bits
- S: Semente utilizada

Você pode executar o binário com os mesmos parâmetros para recriar a execução (Apesar de que esses resultados podem ser relativos a uma versão menos otimizada da aplicação).

A grosso modo, o eixo-y correspondem a profundidade na stack enquanto o eixo-x corrsponde ao tempo ocupado na execução. Para mais informações você pode acessar o projeto do [flamegraph](https://github.com/flamegraph-rs/flamegraph) e checar as documentações da ferramenta.