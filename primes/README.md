# Como achar os dados

Os logs estão nomeados para indicar os parâmetros de execução na forma de M_N_T_random.txt, sendo:

- M: Método utilizado
  - millerrabin: Teste de Miller-Rabin
  - fermat: Teste de Fermat
- N: Número de gerações
- T: Tamanho em bits

No corpo do arquivo você pode encontrar os primos encontrados (`grep º`), os tempos de cada descoberta e tempo médio para cada tentativa até a descoberta (`grep Found`) e no final você pode ver o tempo total da execução e o tempo médio para cada descoberta (`grep Total`).