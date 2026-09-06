#!/usr/bin/env python3
"""
MCP Carousel Server v3.0 - Extensible & Dynamic Hub Architecture
Permite añadir nuevos Hubs, nuevas API Keys, modificar perfiles de recarga/costo
y actualizar la base de datos de cuotas dinámicamente en tiempo de ejecución.
"""

import sys
import json
import time
import os

STATE_FILE = r"C:\Users\eluithi\.gemini\config\mcp_carousel_state.json"
HUBS_META_FILE = r"C:\Users\eluithi\.gemini\config\mcp_hubs_registry.json"

# Perfiles de razonabilidad y límites por defecto para Hubs estándar
DEFAULT_PROFILES = {
    "cerebras": {"tiempo_recarga_horas": 5.0, "max_rpm": 30, "costo_relativo": "gratis_rapido", "prioridad": 1},
    "groq": {"tiempo_recarga_horas": 5.0, "max_rpm": 30, "costo_relativo": "gratis_rapido", "prioridad": 1},
    "sambanova": {"tiempo_recarga_horas": 6.0, "max_rpm": 20, "costo_relativo": "gratis_medio", "prioridad": 2},
    "deepseek": {"tiempo_recarga_horas": 12.0, "max_rpm": 60, "costo_relativo": "pago_ultra_economico_offpeak", "prioridad": 2},
    "gemini": {"tiempo_recarga_horas": 24.0, "max_rpm": 15, "costo_relativo": "gratis_cuota_diaria", "prioridad": 3},
    "kimi": {"tiempo_recarga_horas": 24.0, "max_rpm": 20, "costo_relativo": "pago_moderado", "prioridad": 3},
    "hf": {"tiempo_recarga_horas": 24.0, "max_rpm": 10, "costo_relativo": "gratis_rate_limited", "prioridad": 4},
    "nvidia": {"tiempo_recarga_horas": 48.0, "max_rpm": 10, "costo_relativo": "credito_finito", "prioridad": 4},
    "kaggle": {"tiempo_recarga_horas": 72.0, "max_rpm": 5, "costo_relativo": "computo_pesado_gpu_tpu", "prioridad": 5},
    "colab": {"tiempo_recarga_horas": 12.0, "max_rpm": 10, "costo_relativo": "computo_interactivo", "prioridad": 5},
    "openrouter": {"tiempo_recarga_horas": 24.0, "max_rpm": 20, "costo_relativo": "estandar", "prioridad": 3}
}

def load_hubs_registry():
    if os.path.exists(HUBS_META_FILE):
        try:
            with open(HUBS_META_FILE, "r", encoding="utf-8") as f:
                return json.load(f)
        except Exception:
            pass
    return DEFAULT_PROFILES.copy()

def save_hubs_registry(registry):
    try:
        os.makedirs(os.path.dirname(HUBS_META_FILE), exist_ok=True)
        with open(HUBS_META_FILE, "w", encoding="utf-8") as f:
            json.dump(registry, f, indent=2)
    except Exception as e:
        sys.stderr.write(f"Error guardando registro de hubs: {e}\n")

def load_matrix():
    if os.path.exists(STATE_FILE):
        try:
            with open(STATE_FILE, "r", encoding="utf-8") as f:
                return json.load(f)
        except Exception:
            pass
    return {}

def save_matrix(matrix):
    try:
        os.makedirs(os.path.dirname(STATE_FILE), exist_ok=True)
        with open(STATE_FILE, "w", encoding="utf-8") as f:
            json.dump(matrix, f, indent=2)
    except Exception as e:
        sys.stderr.write(f"Error guardando matriz: {e}\n")

def register_or_update_hub(hub_name: str, tiempo_recarga_horas: float, costo_relativo: str = "personalizado", max_rpm: int = 20, prioridad: int = 3):
    """Permite dar de alta un Hub completamente nuevo o actualizar sus parámetros."""
    registry = load_hubs_registry()
    hub = hub_name.lower().strip()
    
    registry[hub] = {
        "tiempo_recarga_horas": float(tiempo_recarga_horas),
        "max_rpm": int(max_rpm),
        "costo_relativo": str(costo_relativo),
        "prioridad": int(prioridad)
    }
    save_hubs_registry(registry)
    
    # Inicializar espacio en matriz de claves si no existe
    matrix = load_matrix()
    if hub not in matrix:
        matrix[hub] = []
        save_matrix(matrix)
        
    return f"[OK] Hub '{hub}' registrado/actualizado exitosamente con periodo de recarga de {tiempo_recarga_horas}h y perfil '{costo_relativo}'."

def register_key(hub_name: str, api_key: str, custom_cooldown_hours: float = None, metadata: dict = None):
    """Agrega o actualiza una API Key dentro de cualquier Hub (existente o nuevo)."""
    registry = load_hubs_registry()
    matrix = load_matrix()
    hub = hub_name.lower().strip()
    
    # Si el hub no está en el registro, darlo de alta automáticamente con perfil por defecto
    if hub not in registry:
        register_or_update_hub(hub, custom_cooldown_hours or 12.0)
        registry = load_hubs_registry()
        
    hub_profile = registry.get(hub, {"tiempo_recarga_horas": 12.0})
    final_h = custom_cooldown_hours if custom_cooldown_hours is not None else hub_profile.get("tiempo_recarga_horas", 12.0)
    recharge_sec = int(final_h * 3600)
    
    entries = matrix.setdefault(hub, [])
    
    for e in entries:
        if e.get("api_key") == api_key:
            e["tiempo_recarga_segundos"] = recharge_sec
            if metadata:
                e["metadata"] = metadata
            save_matrix(matrix)
            return f"[OK] Clave de {hub} actualizada con periodo de recarga de {final_h}h."
            
    entries.append({
        "hub": hub,
        "api_key": api_key,
        "fecha_consumo": None,
        "tiempo_recarga_segundos": recharge_sec,
        "is_active": True,
        "peticiones_hoy": 0,
        "veces_agotada": 0,
        "metadata": metadata or {}
    })
    save_matrix(matrix)
    return f"[OK] Nueva clave añadida al Hub '{hub}' con periodo de {final_h}h."

def get_optimal_key(hub_name: str, urgency: str = "normal"):
    registry = load_hubs_registry()
    matrix = load_matrix()
    hub = hub_name.lower().strip()
    profile = registry.get(hub, {"tiempo_recarga_horas": 12.0, "costo_relativo": "estandar"})
    now = int(time.time())
    
    entries = matrix.get(hub, [])
    if not entries:
        return None, f"No hay claves registradas en el carrusel para el Hub '{hub}'."

    active_keys = []
    cooldown_keys = []

    for entry in entries:
        consumed_at = entry.get("fecha_consumo")
        recharge_sec = entry.get("tiempo_recarga_segundos", int(profile["tiempo_recarga_horas"] * 3600))
        
        if consumed_at is not None:
            if now >= consumed_at + recharge_sec:
                # AUTO-RECUPERACIÓN: El tiempo de recarga ya concluyó
                entry["fecha_consumo"] = None
                entry["is_active"] = True
                active_keys.append(entry)
            else:
                eta = (consumed_at + recharge_sec) - now
                entry["is_active"] = False
                entry["eta_segundos"] = eta
                cooldown_keys.append(entry)
        else:
            entry["is_active"] = True
            active_keys.append(entry)

    # Ordenar en enfriamiento por ETA más próximo
    cooldown_keys.sort(key=lambda x: x.get("eta_segundos", 999999))
    matrix[hub] = active_keys + cooldown_keys
    save_matrix(matrix)

    if active_keys:
        chosen = active_keys[0]
        chosen["ultima_peticion_timestamp"] = now
        chosen["peticiones_hoy"] = chosen.get("peticiones_hoy", 0) + 1
        save_matrix(matrix)
        
        return chosen.get("api_key"), f"[OK] Clave activa para {hub} (Costo: {profile.get('costo_relativo')}, Recarga: {profile.get('tiempo_recarga_horas')}h)."
    else:
        next_ready = cooldown_keys[0]
        eta_m = next_ready.get("eta_segundos", 0) // 60
        eta_h = eta_m // 60
        eta_m_rem = eta_m % 60
        return None, f"[COOLDOWN ACTIVO] Todas las claves de {hub} están descansando. Próxima disponible en {eta_h}h {eta_m_rem}m."

def mark_exhausted(hub_name: str, api_key: str, reason: str = "HTTP 429 Quota Exceeded"):
    registry = load_hubs_registry()
    matrix = load_matrix()
    hub = hub_name.lower().strip()
    now = int(time.time())
    
    entries = matrix.get(hub, [])
    found = False
    for entry in entries:
        if entry.get("api_key") == api_key:
            entry["fecha_consumo"] = now
            entry["is_active"] = False
            entry["ultimo_motivo_paro"] = reason
            entry["veces_agotada"] = entry.get("veces_agotada", 0) + 1
            found = True
            break
            
    if found:
        save_matrix(matrix)
        recharge_h = registry.get(hub, {}).get("tiempo_recarga_horas", 12.0)
        return True, f"Clave de {hub} puesta en enfriamiento a las {time.ctime(now)}. Se reactivará automáticamente en {recharge_h} horas."
    return False, "Clave no encontrada."

def full_status_report():
    registry = load_hubs_registry()
    matrix = load_matrix()
    now = int(time.time())
    
    report = {
        "timestamp": time.ctime(now),
        "total_hubs_registrados": len(registry),
        "hubs": {}
    }
    
    for hub, profile in registry.items():
        entries = matrix.get(hub, [])
        activas = 0
        cooldown = 0
        detail = []
        
        for e in entries:
            consumed = e.get("fecha_consumo")
            recharge_sec = e.get("tiempo_recarga_segundos", int(profile.get("tiempo_recarga_horas", 12.0) * 3600))
            if consumed is not None:
                if now >= consumed + recharge_sec:
                    estado = "🟢 RECUPERADA (Lista)"
                    eta = "Disponible ahora"
                    activas += 1
                else:
                    estado = "🔴 ENFRIAMIENTO (429)"
                    rem = (consumed + recharge_sec) - now
                    eta = f"{rem//3600}h {(rem%3600)//60}m restantes"
                    cooldown += 1
            else:
                estado = "🟢 ACTIVA (Fresca)"
                eta = "Disponible ahora"
                activas += 1
                
            detail.append({
                "clave": e.get("api_key", "")[:8] + "..." + e.get("api_key", "")[-4:],
                "estado": estado,
                "fecha_consumo": time.ctime(consumed) if consumed else "Nunca",
                "tiempo_recarga_horas": recharge_sec / 3600.0,
                "tiempo_restante": eta,
                "peticiones_hoy": e.get("peticiones_hoy", 0),
                "veces_agotada": e.get("veces_agotada", 0),
                "metadata": e.get("metadata", {})
            })
            
        report["hubs"][hub] = {
            "perfil": profile,
            "claves_activas": activas,
            "claves_en_enfriamiento": cooldown,
            "claves": detail
        }
    return report

def main():
    while True:
        line = sys.stdin.readline()
        if not line:
            break
        try:
            req = json.loads(line)
            req_id = req.get("id")
            method = req.get("method")
            
            if method == "initialize":
                res = {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {
                        "protocolVersion": "2024-11-05",
                        "capabilities": {"tools": {}},
                        "serverInfo": {"name": "mcp-carousel-orchestrator", "version": "3.0.0"}
                    }
                }
            elif method == "tools/list":
                res = {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {
                        "tools": [
                            {
                                "name": "carousel_get_key",
                                "description": "Obtiene la API key activa óptima para cualquier hub.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "hub": {"type": "string", "description": "Nombre del hub"}
                                    },
                                    "required": ["hub"]
                                }
                            },
                            {
                                "name": "carousel_mark_exhausted",
                                "description": "Marca una clave como consumida (429), iniciando su enfriamiento.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "hub": {"type": "string"},
                                        "api_key": {"type": "string"},
                                        "reason": {"type": "string"}
                                    },
                                    "required": ["hub", "api_key"]
                                }
                            },
                            {
                                "name": "carousel_register_new_hub",
                                "description": "Registra un NUEVO Hub o actualiza sus parámetros (periodo de recarga, costo, RPM).",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "hub": {"type": "string", "description": "Nombre único del nuevo Hub (ej: mistral, cohere, lambda_cloud)"},
                                        "recharge_hours": {"type": "number", "description": "Periodo de recarga en horas (ej: 6.0, 24.0)"},
                                        "cost_profile": {"type": "string", "description": "Perfil de costo (gratis, pago_bajo, computo_pesado, etc.)"},
                                        "max_rpm": {"type": "integer", "description": "Límite de peticiones por minuto (RPM)"}
                                    },
                                    "required": ["hub", "recharge_hours"]
                                }
                            },
                            {
                                "name": "carousel_register_key",
                                "description": "Agrega una nueva API Key o datos adicionales a cualquier Hub (existente o nuevo).",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {
                                        "hub": {"type": "string"},
                                        "api_key": {"type": "string"},
                                        "recharge_hours": {"type": "number"},
                                        "metadata": {"type": "object", "description": "Datos adicionales personalizados (ej: cuenta, usuario, notas)"}
                                    },
                                    "required": ["hub", "api_key"]
                                }
                            },
                            {
                                "name": "carousel_reasonability_report",
                                "description": "Genera el reporte completo de todos los Hubs, claves, estados de enfriamiento y datos dinámicos.",
                                "inputSchema": {
                                    "type": "object",
                                    "properties": {}
                                }
                            }
                        ]
                    }
                }
            elif method == "tools/call":
                params = req.get("params", {})
                name = params.get("name")
                args = params.get("arguments", {})
                
                if name == "carousel_get_key":
                    hub = args.get("hub", "")
                    key, msg = get_optimal_key(hub)
                    text_out = json.dumps({"hub": hub, "api_key": key, "info": msg}, indent=2)
                elif name == "carousel_mark_exhausted":
                    hub = args.get("hub", "")
                    k = args.get("api_key", "")
                    r = args.get("reason", "429 Quota Exceeded")
                    ok, msg = mark_exhausted(hub, k, r)
                    text_out = json.dumps({"success": ok, "message": msg}, indent=2)
                elif name == "carousel_register_new_hub":
                    hub = args.get("hub", "")
                    h = float(args.get("recharge_hours", 12.0))
                    c = args.get("cost_profile", "personalizado")
                    rpm = int(args.get("max_rpm", 20))
                    msg = register_or_update_hub(hub, h, c, rpm)
                    text_out = json.dumps({"message": msg}, indent=2)
                elif name == "carousel_register_key":
                    hub = args.get("hub", "")
                    k = args.get("api_key", "")
                    h = args.get("recharge_hours")
                    meta = args.get("metadata", {})
                    msg = register_key(hub, k, float(h) if h is not None else None, meta)
                    text_out = json.dumps({"message": msg}, indent=2)
                elif name == "carousel_reasonability_report":
                    rep = full_status_report()
                    text_out = json.dumps(rep, indent=2)
                else:
                    text_out = f"Herramienta desconocida: {name}"
                    
                res = {
                    "jsonrpc": "2.0",
                    "id": req_id,
                    "result": {
                        "content": [{"type": "text", "text": text_out}]
                    }
                }
            else:
                res = {"jsonrpc": "2.0", "id": req_id, "result": {}}
                
            sys.stdout.write(json.dumps(res) + "\n")
            sys.stdout.flush()
        except Exception as e:
            err_res = {"jsonrpc": "2.0", "id": None, "error": {"code": -32603, "message": str(e)}}
            sys.stdout.write(json.dumps(err_res) + "\n")
            sys.stdout.flush()

if __name__ == "__main__":
    main()
