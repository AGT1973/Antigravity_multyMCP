---
name: carousel-fix-and-repair
description: >-
  Invoca y repara en caliente el estado del carrusel de claves y hubs MCP.
  Utilízalo ante cualquier bloqueo de cuota, error de ruta o para auditar disponibilidad de cómputo.
---

# Skill: Reparación y Auto-Sanación de Carrusel MCP

Cuando el alumno o el orquestador detecte que un Hub da error de cuota o clave agotada:

1. **Auto-Diagnóstico:**
   - Invoca la herramienta `carousel_reasonability_report` del plugin `mcp-carousel-orchestrator`.
   - Muestra al usuario/alumno cuántas claves activas quedan y cuánto tiempo exacto resta para las que están descansando.

2. **Rotación Rápida:**
   - Si una clave da error `429`, invoca inmediatamente `carousel_mark_exhausted(hub="...", api_key="...")`.
   - Recupera la siguiente clave activa con `carousel_get_key(hub="...")`.

3. **Alta de Nueva Llave en Caliente:**
   - Si el alumno dispone de una clave fresca:
   - Invoca `carousel_register_key(hub="...", api_key="...")`.
