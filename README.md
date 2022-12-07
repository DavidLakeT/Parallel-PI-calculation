# Procesos
## Primera Evaluación - Planificación de procesos

Para el primer trabajo puede seleccionar entre dos alternativas diferentes: construir un simulador de procesos o escribir un programa que utilice hilos para calcular el valor de $\pi$ de manera paralela. A continuación, encontrarán la definición de ambas alternativas

## Simulador de planificación de procesos
Una de las actividades básicas que realiza un sistema operativo es la planificación de los procesos. Esta actividad es fundamental porque permite compartir el uso del procesador entre varios procesos y, así, optimizar algún indicador definido para la eficiencia del sistema. Esta alternativa consiste en implementar un simulador de planificación de procesos. A continuación se describen los componentes más importantes del simulador[^1]
[^1]: Esta lista no es exahustiva

### Descripción de los procesos
La lista de los procesos debe ser leida de un archivo texto que tiene la siguiente estructura:
- La primera línea es un número entero $n$ que representa cuantos procesos están en el archivo
- Cada una de las siguientes $n$ líneas representa un proceso y contiene la siguiente información:
  - El `id` del proceso
  - El tipo de proceso `tp`, puede ser `batch` o `interactivo`
  - La prioridad `P` ( $\mathcal{P}$ ) del proceso. La prioridad es un valor entero no negativo. Mientras mayor sea el valor de $p$, mayor es la prioridad.
  - El instante de tiempo `t0` ( $t_0$ ) en el que el proceso se activa y puede comenzar a ejecutarse, i.e. el instante a partir del cual el proceso se incluye en la planficación
  - El número de unidades de tiempo `T` ( $T$ ) que toma la ejecución total del proceso[^2].
[^2]: Este tiempo no incluye la duración de las operacione de entrada/salida
  - El período `p` ( $p$ ) de las operaciones de entrada salida. Por ejemplo, si $p = 3$, entonces quiere decir que cada vez que el tiempo acumulado de ejecución del proceso es múltiplo de tres (3), realiza una operación de entrada/salida[^3]. Si $p > T$, entonces el proceso no realiza operaciones de entrada/salida.
[^3]: Recuerde que, gracias a la incorporación de tecnologías como DMA ( *Direct Memory Access* ), el procesador no interviene en el desarrollo de la operación de entrada/salida, así que el proceso se suspende y no es tenido en cuenta para la planificación.
  - La duración `D` ( $\mathcal{D}$ ) de cada operación de entrada salida. 

### Función de selección del siguiente proceso
Se debe proporcionar una función, que sea modificable fácilmente, que se utilizará para seleccionar a cuál proceso se le asignará el procesador. El simulador debe llamar esta función para todos los procesos *activos*, es decir, que se puedan programar en ese momento, y seleccionar aquel para el cuál la función dé el máximo valor. Algunas variables que deben ser tenidas en cuenta para la implementación de dicha función son: 
- La duración total $T$ del proceso.
- El número de unidades de tiempo que faltan para que el proceso termine su ejecución.
- El tipo de proceso.
- La prioridad $\mathcal{P}$ del proceso.
- El período $p$ de las operaciones de entrada/salida.
- La duración $\mathcal{D}$ de las operaciones de entrada/salida

Algunos ejemplos de los criterios que puede implementar esta función son: el proceso que más tiempo le falta para terminar, el proceso que le falta menos tiempo para terminar, el proceso de mayor prioridad, el proceso que vaya a realizar la siguiente operación de entrada/salida más pronto, etc. 

### Función de eficiencia del sistema
El objetivo del planificador es maximizar la eficiencia. Para ello, debe calcular esta función para cada uno de los procesos e imprimir el promedio del valor de la función. Los parámetros que debe recibir la función son:
- Instante de tiempo $t_0$ en el cuál el proceso se activa.
- Instante de tiempo $t_f$ en el cuál el proceso terminó su ejecución.
- Unidades de tiempo $T$ que toma la ejecución del proceso.

Algunos ejemplos de los indicadores que puede implementar esta función son: *walltime*[^4], relación entre el *walltime* y el número de unidades de tiempo $T$ que toma la ejecución del proceso, etc. 
[^4]: El *walltime* para un proceso es $t_f - t_0$

### Funcionamiento del simulador
El funcionamiento del simulador es simple: 
1. Lee la información de todos los procesos.
2. Evalúa la función de selección para todos los procesos activos y, con base en el resultado, selecciona el siguiente proceso a ejecutar. 
3. Cada vez que un proceso suspenda su ejecución, repite el paso 2[^5].
[^5]: Recuerde que un proceso se puede suspender porque realiza una operación de entrada/salida o porque se venció el *quantum* de tiempo. El *quantum* es un parámetro del simulador. 
4. Cada vez que un proceso termine su ejecución, evalua la función de eficiencia
5. Cuando todos los procesos terminen, debe imprimir la siguiente información para cada proceso: 
    - Identificador `id` del proceso
    - Tiempo de inicio $t_0$
    - Tiempo de finalización $t_f$
    - Valor de la función de eficiencia
6. Imprimir el valor promedio de la función de eficiencia

### Bonificación
Si además de realizar la simulación de la ejecución de los procesos, desarrolla un algoritmo que calcule cuál sería la planificación que maximizaría la función de eficiencia, tendrá una bonificación del 20% (una unidad) en el segundo trabajo práctico. Este algoritmo puede ser visto como una versión en bloques del algoritmo de planificación, pues, a diferencia del simulador que utiliza un algoritmo en línea, utiliza toda la información de todos los procesos. 

## Cálculo paralelo de $\pi$
Uno de los retos importantes en la computación actual, es poder aprovechar a fondo las capacidades que ofrece el *hardware* moderno. El uso de múltiples procesadores en paralelo ha permitido mantener, al menos temporalmente, la promesa de la ley de moore[^6] de duplicar la capacidad computacional cada 18 meses. Con la aparición de los sistemas multiprocesador, aparecen los hilos (*threads*) como un mecanismo más eficiente que los procesos para aprovechar la capacidad de operar en paralelo de las máquinas modernas. Esta alternativa consiste en utilizar múltiples hilos de ejecución para calcular el valor de $\pi$. 
[^6]: [https://es.wikipedia.org/wiki/Ley_de_Moore](https://es.wikipedia.org/wiki/Ley_de_Moore)

Para calcular $\pi$ de manera paralela, se puede partir de la siguiente expresion: 

$$\int_{0}^{1}\frac{4}{1+x^2}dx=\pi$$

Esta integral se puede aproximar de manera discreta de la siguiente manera: Sea $\Delta = \frac{1}{N}$ y $x_i = (i+0.5)\Delta$ con $i = 0, 1, 2, ..., N-1$, donde $N$ es el número de rectángulos en los que vamos a dividir el intervalo $[0,1]$. Entonces, $\pi$ se puede aproximar como 

$$\sum_{i=0}^{N-1}\frac{4}{1+x_i^2}\Delta \cong \pi$$

Implemente un programa que utilice múltiples hilos de ejecución para calcular el valor de $\pi$ en paralelo. 
