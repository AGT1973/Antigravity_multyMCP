#!/usr/bin/env python3
"""
hub_client.py — Cliente pedagógico para gestión de carrusel de API keys y catálogo de modelos.
Uso desde Python:
    from hub_client import chat, status_report
    respuesta = chat(hub="cerebras", prompt="Explicame que es la IA")
    respuesta = chat(hub="auto", prompt="Hola")
    print(status_report())

Uso desde consola:
    python hub_client.py cerebras "Explicame que es la IA"
    python hub_client.py auto "Hola"
    python hub_client.py status
    python hub_client.py add_key cerebras TU_API_KEY
"""

import json
import time
import sys
import urllib.request
import urllib.error
from pathlib import Path
from typing import Optional, Tuple, List

BASE_DIR = Path(__file__).parent
CONFIG_FILE = BASE_DIR / "student_config.json"
TEMPLATE_FILE = BASE_DIR / "student_config_TEMPLATE.json"
CATALOG_FILE = BASE_DIR / "hubs_master_catalog.json"

def load_config() -> dict:
    """Carga la configuración del alumno o la plantilla si falta el archivo real."""
    if CONFIG_FILE.exists():
        with open(CONFIG_FILE, "r", encoding="utf-8") as f:
            return json.load(f)
    if TEMPLATE_FILE.exists():
        with open(TEMPLATE_FILE, "r", encoding="utf-8") as f:
            return json.load(f)
    raise FileNotFoundError(
        f"No se encontró ni student_config.json ni student_config_TEMPLATE.json en {BASE_DIR}"
    )

def save_config(cfg: dict) -> None:
    """Guarda el JSON de configuración (sobrescribe student_config.json)."""
    with open(CONFIG_FILE, "w", encoding="utf-8") as f:
        json.dump(cfg, f, indent=2, ensure_ascii=False)

def _now() -> float:
    return time.time()

def get_active_key(cfg: dict, hub: str) -> Tuple[Optional[str], Optional[str]]:
    hub_cfg = cfg.get("hubs", {}).get(hub)
    if not hub_cfg:
        return None, f"[ERROR] Hub '{hub}' no está configurado."
    keys = hub_cfg.get("keys", [])
    if not keys:
        return None, f"[ERROR] Hub '{hub}' no tiene claves configuradas."
    now = _now()
    activas: List[dict] = []
    cooldowns: List[Tuple[dict, float]] = []
    default_cool = hub_cfg.get("cooldown_hours", 5.0)
    for k in keys:
        exhausted = k.get("exhausted_at")
        cd_h = k.get("cooldown_hours", default_cool)
        cd_s = cd_h * 3600
        if exhausted is None:
            activas.append(k)
        elif now >= exhausted + cd_s:
            k["exhausted_at"] = None
            activas.append(k)
        else:
            eta = (exhausted + cd_s) - now
            cooldowns.append((k, eta))
    save_config(cfg)
    if activas:
        activas.sort(key=lambda x: x.get("last_used", 0))
        chosen = activas[0]
        chosen["last_used"] = now
        chosen["peticiones_total"] = chosen.get("peticiones_total", 0) + 1
        save_config(cfg)
        return chosen["api_key"], None
    cooldowns.sort(key=lambda x: x[1])
    eta_sec = cooldowns[0][1]
    h, m = int(eta_sec // 3600), int((eta_sec % 3600) // 60)
    tiempo = f"{h}h {m}m" if h else f"{m}m"
    msg = (
        f"[ALERTA DE CUOTA] Todas las claves del hub '{hub}' están agotadas.\n"
        f"Próxima clave disponible en {tiempo}."
    )
    return None, msg

def mark_exhausted(cfg: dict, hub: str, api_key: str, reason: str = "429") -> None:
    for k in cfg.get("hubs", {}).get(hub, {}).get("keys", []):
        if k.get("api_key") == api_key:
            k["exhausted_at"] = _now()
            k["last_error"] = reason
            k["veces_agotada"] = k.get("veces_agotada", 0) + 1
            break
    save_config(cfg)

def get_optimal_hub(cfg: dict) -> Optional[str]:
    now = _now()
    candidates: List[Tuple[str, float]] = []
    for hub_name, hub_cfg in cfg.get("hubs", {}).items():
        default_cd = hub_cfg.get("cooldown_hours", 12.0)
        for k in hub_cfg.get("keys", []):
            exhausted = k.get("exhausted_at")
            cd_h = k.get("cooldown_hours", default_cd)
            if exhausted is None or now >= exhausted + cd_h * 3600:
                candidates.append((hub_name, cd_h))
                break
    if not candidates:
        return None
    candidates.sort(key=lambda x: x[1])
    return candidates[0][0]

def _openai_call(base_url: str, api_key: str, model: str, messages: List[dict], **kwargs) -> str:
    payload = {"model": model, "messages": messages}
    payload.update(kwargs)
    data = json.dumps(payload).encode("utf-8")
    req = urllib.request.Request(
        url=base_url.rstrip("/") + "/chat/completions",
        data=data,
        headers={
            "Content-Type": "application/json",
            "Authorization": f"Bearer {api_key}",
            "User-Agent": "HubClient-Pedagogico",
        },
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=45) as resp:
        resp_data = json.load(resp)
        return resp_data["choices"][0]["message"]["content"]

def chat(
    hub: str,
    prompt: str,
    system: Optional[str] = None,
    max_retries: Optional[int] = None,
    **kwargs,
) -> str:
    cfg = load_config()
    if hub == "auto":
        hub = get_optimal_hub(cfg)
        if not hub:
            return "[ALERTA TOTAL] Todos los hubs están agotados. Agrega más claves o espera cooldown."
    hub_cfg = cfg.get("hubs", {}).get(hub)
    if not hub_cfg:
        return f"[ERROR] Hub '{hub}' no existe. Disponible: {list(cfg.get('hubs', {}).keys())}"
    model = hub_cfg.get("model", "")
    base_url = hub_cfg.get("base_url", "")
    compatible = hub_cfg.get("compatible_with", "openai")
    retries = max_retries if max_retries is not None else max(len(hub_cfg.get("keys", [])), 1)
    msgs = []
    if system:
        msgs.append({"role": "system", "content": system})
    msgs.append({"role": "user", "content": prompt})
    for intento in range(retries):
        api_key, alerta = get_active_key(cfg, hub)
        if not api_key:
            return alerta
        try:
            if compatible == "openai":
                return _openai_call(base_url, api_key, model, msgs, **kwargs)
            else:
                return f"[INFO] Hub '{hub}' usa protocolo '{compatible}' no implementado."
        except urllib.error.HTTPError as e:
            codigo = e.code
            body = e.read().decode("utf-8", errors="replace")[:300]
            if codigo == 404:
                return (
                    f"[MODELO DESACTUALIZADO] Hub '{hub}' rechazó modelo '{model}' (404).\n"
                    f"Actualiza el campo 'model' en student_config.json."
                )
            if codigo in (429, 401, 403):
                mark_exhausted(cfg, hub, api_key, reason=str(codigo))
                if intento < retries - 1:
                    continue
                return f"[CUOTA AGOTADA] Todas las claves de '{hub}' fallaron ({codigo})."
            return f"[ERROR HTTP {codigo}] Hub '{hub}': {body}"
        except Exception as exc:
            return f"[ERROR] Hub '{hub}': {type(exc).__name__}: {exc}"
    return f"[ERROR] No se pudo completar tras {retries} intentos en hub '{hub}'."

def add_key(hub: str, api_key: str, cooldown_hours: Optional[float] = None, notes: str = "") -> str:
    cfg = load_config()
    hubs = cfg.setdefault("hubs", {})
    if hub not in hubs:
        hub_info = {}
        if CATALOG_FILE.exists():
            with open(CATALOG_FILE, "r", encoding="utf-8") as f:
                catalog = json.load(f)
            hub_info = catalog.get("hubs", {}).get(hub, {})
        hubs[hub] = {
            "model": hub_info.get("model", "PENDIENTE_VERIFICACION"),
            "base_url": hub_info.get("base_url", ""),
            "compatible_with": hub_info.get("compatible_with", "openai"),
            "cooldown_hours": cooldown_hours or hub_info.get("cooldown_hours", 12.0),
            "keys": [],
        }
    hub_cfg = hubs[hub]
    for k in hub_cfg["keys"]:
        if k.get("api_key") == api_key:
            return f"[INFO] La clave ya existe en hub '{hub}'."
    hub_cfg["keys"].append({
        "api_key": api_key,
        "cooldown_hours": cooldown_hours or hub_cfg.get("cooldown_hours", 12.0),
        "exhausted_at": None,
        "last_used": 0,
        "peticiones_total": 0,
        "veces_agotada": 0,
        "notes": notes,
    })
    save_config(cfg)
    return f"[OK] Clave añadida a hub '{hub}'."

def status_report() -> str:
    cfg = load_config()
    now = _now()
    lines = ["===== Estado del Carrusel de Hubs =====", f"Fecha: {time.strftime('%Y-%m-%d %H:%M:%S')}", ""]
    for hub_name, hub_cfg in cfg.get("hubs", {}).items():
        lines.append(f"[{hub_name.upper()}] modelo: {hub_cfg.get('model', 'N/A')}")
        keys = hub_cfg.get("keys", [])
        if not keys:
            lines.append("  Sin claves configuradas")
            continue
        for i, k in enumerate(keys, 1):
            exhausted = k.get("exhausted_at")
            cd_h = k.get("cooldown_hours", hub_cfg.get("cooldown_hours", 5.0))
            cd_s = cd_h * 3600
            masked = f"{k.get('api_key','')[:8]}...{k.get('api_key','')[-4:]}"
            status = "ACTIVA"
            extra = ""
            if exhausted is not None:
                if now >= exhausted + cd_s:
                    status = "RECUPERADA"
                else:
                    status = "COOLDOWN"
                    rem = (exhausted + cd_s) - now
                    h, m = int(rem // 3600), int((rem % 3600) // 60)
                    extra = f" -> disponible en {h}h {m}m"
            lines.append(f"  [{i}] {masked} | {status}{extra} | usos:{k.get('peticiones_total',0)}")
    return "\n".join(lines)

if __name__ == "__main__":
    args = sys.argv[1:]
    if not args or args[0] in ("-h", "--help", "help"):
        print(__doc__)
        sys.exit(0)
    cmd = args[0]
    if cmd == "status":
        print(status_report())
    elif cmd == "add_key" and len(args) >= 3:
        print(add_key(args[1], args[2], notes=args[3] if len(args) > 3 else ""))
    elif len(args) >= 2:
        hub_arg = args[0]
        prompt_arg = args[1]
        system_arg = args[2] if len(args) > 2 else None
        print(chat(hub_arg, prompt_arg, system=system_arg))
    else:
        print("Uso: python hub_client.py <hub|auto> \"<prompt>\" [system] | status | add_key <hub> <api_key>")
        sys.exit(1)
