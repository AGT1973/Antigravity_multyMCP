# INFORME DE AUDITORÍA PROFUNDA: QWEN 2.5 BULLDOG (HALLAZGOS DE SILICIO Y MEMORIA)

> **Documento Consolidado de Ingesta Analítica (Regla 19 + Regla 16)**
> **Origen:** `E:\POLYDIM_EINSOF\ENTREGA_2026_09_13_V716\respuestas\qwen.md` (2,448 líneas)

---

## 1. INTRODUCCIÓN Y ENFOQUE

Mientras que DeepSeek R1 enfocó su crítica en la matemática diferencial abstracta de la hiperesfera (geodesias, transporte paralelo, curvatura), **Qwen 2.5 atacó el código a nivel de silicio, alineación de memoria, registros de CPU, manejo de páginas del SO y concurrencia FFI**.

Qwen realizó un asedio en dos pasadas (la segunda bajo la postura "Modo Bulldog Extremo"), descubriendo 6 vulnerabilidades críticas de hardware que habrían provocado la falla catastrófica del sistema antes de que la matemática pudiera siquiera ejecutarse.

---

## 2. LOS 6 FALLOS FATALES DE SILICIO Y HARDWARE DESCUBIERTOS POR QWEN

### 1. El Asesino del Zero-Copy: Falta de Alineación de Página (4KB Page Alignment)
- **Fallo:** Los arrays de Numpy asignados con `np.zeros()` o `malloc` convencional no garantizan estar alineados a límites de página de 4KB (`4096 bytes`). Los adaptadores de red RDMA de hardware (como Mellanox ConnectX o Intel RoCEv2) **rechazan categóricamente** (`EINVAL`) cualquier región de memoria enviada a `ibv_reg_mr` que no esté alineada a la página del kernel del SO.
- **Solución C++:** Asignar la memoria de los tensores en C++ usando `mmap(NULL, length, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0)`, el cual garantiza alineación exacta a 4KB por diseño del SO.

### 2. Envenenamiento de `lkey` en RDMA (Local Key Criptográfico)
- **Fallo:** En el wrapper Rust (`pmtp_kernel.rs.txt`), la variable `lkey` estaba hardcodeada en `0`. En la arquitectura InfiniBand/RoCEv2, la NIC exige una clave local (`lkey`) para validar los permisos de lectura DMA. Enviar `0` hace que la NIC descarte el paquete o lea memoria corrupta.
- **Solución Rust/C++:** Exponer una estructura `PmtpMR` compatible con `#[repr(C)]` entre C++ y Rust para extraer dinámicamente el `lkey` registrado por el HCA.

### 3. Asfixia por TLB Thrashing y OOM en el Espacio Tangente
- **Fallo:** En `pmtp_monolito.py`, la operación `projections = pts - np.outer(dots, center)` creaba un array temporal de `(n_k, 10000)` en cada iteración de cluster. A $D=10,000$, esto asignaba 4MB de RAM efímera por cluster por iteración, desbordando la caché L1 (32KB) y provocando un *TLB Thrashing* (Translation Lookaside Buffer) masivo con congelamientos del Garbage Collector (GC).
- **Solución Python:** Reemplazar `np.outer` por broadcasting in-place: `projections = pts.copy(); projections -= (dots[:, np.newaxis] * center)`.

### 4. Bloqueo de DMA Asíncrono por el GC de Python
- **Fallo:** Las transferencias RDMA operan por Direct Memory Access (DMA) asíncrono. Si el recolector de basura de Python (GC) se activa a mitad de una transferencia DMA, puede reubicar o liberar el buffer en RAM, provocando un *Segmentation Fault* o corrupción silenciosa.
- **Solución Python:** Desactivar temporalmente el recolector de basura (`gc.disable()`) durante el pasaje de punteros `ctypes` y mantener referencias fuertes al array.

### 5. Fuga de Memoria (Memory Leak) en el Retriever Tensorial
- **Fallo:** En `LatentRetriever.query()`, el modelo `SentenceTransformer('all-MiniLM-L6-v2')` se instanciaba dentro de la función de consulta. Cada búsqueda volvía a cargar 80MB de modelo en RAM, provocando un colapso por OOM en cuestión de horas.
- **Solución Python:** Convertir la carga del modelo en un Singleton instanciado una sola vez en el `__init__`.

### 6. Subnormales y Divisiones por Cero en FP32
- **Fallo:** En `normalize`, la condición `norms[norms == 0] = 1.0` no protege contra números subnormales en precisión FP32 (valores menores a $1.175494 \times 10^{-38}$). La división por un subnormal genera `NaN` o `Inf` que se propagan destruyendo el manifold.
- **Solución Python:** Usar un umbral de seguridad basado en `np.finfo(np.float32).tiny` y `np.where`.

---

## 3. COMPARATIVA TÁCTICA: DEEPSEEK R1 VS. QWEN 2.5

| Dimensión de Análisis | DeepSeek R1 (Ronda 1 y 2) | Qwen 2.5 (Ronda 1 y 2) |
| :--- | :--- | :--- |
| **Foco Principal** | Geometría diferencial, curvatura de $S^{D-1}$, transporte paralelo y el cuello de botella de atención $O(n^2 D)$. | Física del silicio, alineación de páginas a 4KB (`mmap`), TLB thrashing, GC de Python y llaves `lkey` de RDMA. |
| **Nivel de Abstracción** | Topológico / Académico. | Bajo Nivel / Sistemas Operativos / Kernel. |
| **Diagnóstico de Fails** | Demostró matemáticamente por qué el transporte paralelo falla en $D=10,000$. | Encontró por qué el código actual fallaba en la NIC antes de ejecutar cualquier matemática. |
| **Complementariedad** | Nos dio la dirección del mapa abstracto. | Nos dio las correcciones exactas de código de silicio. |

---

## 4. CONCLUSIÓN Y ESTADO DEL PROTOCOLO DE INGESTA

La auditoría de Qwen 2.5 complementa perfectamente la victoria teórica lograda contra DeepSeek R1. 
- DeepSeek nos obligó a abandonar los experimentos geométricos complejos (transporte paralelo) en favor del consenso directo.
- Qwen nos dio las 6 correcciones exactas para que el código en C++, Rust y Python sea **industrialmente inquebrantable a nivel de hardware**.

De acuerdo con la **Regla 19 (Veto de Código)**, estas correcciones permanecen consolidadas en este informe sin modificar una sola línea de código fuente hasta que el usuario emita la orden explícita de liberación.
