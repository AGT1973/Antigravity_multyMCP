---
name: mcp-carousel-manager
description: >-
  Orquesta, rota y monitorea en carrusel las API Keys y Servidores MCP utilizando 4 campos:
  Hub, API Key, Fecha de Terminación (Consumo) y Tiempo de Recarga (Cooldown).
---

# Skill: MCP Carousel Manager

Esta habilidad permite al agente dedicado gestionar la matriz temporal de cuotas y rotación de API Keys en tiempo real.

## Los 4 Campos Esenciales

Cada entrada de la matriz contiene obligatoriamente:
1. **Hub / Proveedor (`hub`):** Identificador del endpoint (ej. `groq`, `cerebras`, `gemini`, `nvidia`, `kimi`, `deepseek`, `kaggle`, `colab`).
2. **API Key (`api_key`):** Token de acceso autenticado.
3. **Fecha de Consumo (`fecha_consumo`):** Timestamp exacto cuando la clave devolvió `HTTP 429` (Quota Exceeded).
4. **Tiempo de Recarga (`tiempo_recarga`):** Periodo en horas necesario para que el proveedor reponga la cuota (ej. 5h para Groq, 24h para Gemini, 72h para Kaggle).

---

## Procedimiento Operativo del Agente

### 1. Obtener la Llave Óptima para una Petición
Antes de disparar una consulta a un modelo o lanzar un kernel a la nube:
- Invocar `carousel_get_key(hub="groq")`.
- El servidor verifica si la clave actual está activa. Si está en cooldown pero ya transcurrieron las horas necesarias, **la auto-recupera inmediatamente** y la coloca al frente de la cola.
- Si todas están agotadas, reporta el tiempo exacto en minutos para la próxima clave disponible.

### 2. Manejo de Errores 429 (Rotación Inmediata)
Ante cualquier respuesta de cuota agotada (`HTTP 429`):
- Invocar `carousel_mark_exhausted(hub="groq", api_key="...")`.
- Esto estampa la fecha actual e inicia el contador de enfriamiento.
- El carrusel salta automáticamente a la siguiente clave disponible del carrusel sin generar tiempos muertos.

### 3. Reporte de Estado y Auditoría
Para consultar el tablero de estado de todas las llaves y tiempos restantes:
- Invocar `carousel_status_report()`.
