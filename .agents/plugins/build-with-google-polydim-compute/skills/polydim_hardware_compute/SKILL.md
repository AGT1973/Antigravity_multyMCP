---
name: polydim-hardware-compute
description: >-
  Orquesta evaluaciones asintóticas en CPU, GPU (NVIDIA T4/A100) y TPU (v3-8)
  en Google Colab y Kaggle, con gestión temporal de cuotas y rotación por enfriamiento.
---

# Skill: POLYDIM Hardware Compute Suite

Esta habilidad guía al agente para ejecutar cálculos tensoriales pesados ($D \ge 10,000$ o $10^9$ elementos) en aceleradores remotos.

## Cuándo Usar

- Al ejecutar benchmarks de preservación de isometría en altas dimensiones ($D \ge 10,000$).
- Al someter a prueba operadores KPON, rotores Clifford o descomposiciones de Kronecker en vectores de 1 Billón ($10^9$ elementos).
- Al realizar auditorías empíricas sin consumir recursos locales de la máquina física.

## Flujo de Trabajo

1. **Kaggle Compute (Desatendido):**
   - Usar `kaggle_check_auth` para verificar credenciales (`tradingnewtech` o `polydimailenguage`).
   - Usar `kaggle_run_benchmark` especificando `accelerator='gpu'` o `accelerator='tpu'`.
   - Recuperar logs crudos mediante `kaggle_get_results`.

2. **Google Colab Live (Tiempo Real):**
   - Configurar el túnel con `colab_set_tunnel` usando la URL pública de Cloudflare.
   - Probar disponibilidad con `colab_ping_hardware`.
   - Enviar bloques de tensores en JAX/PyTorch mediante `colab_eval_code`.

3. **Gestión Temporal de Cuotas (Token Matrix):**
   - El sistema reordena las API Keys por fecha de consumo y tiempo de recarga (ej. 5h para Groq, 24h para Gemini, 72h para Kaggle/TPU).
   - Si una llave da 429, pasa al final de la cola hasta cumplir su periodo de reposición.
