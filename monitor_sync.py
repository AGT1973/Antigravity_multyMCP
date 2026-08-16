#!/usr/bin/env python3
"""
Monitor de Sincronización - Antigravity (Configuración MCP)
===========================================================
Asegura que la infraestructura multi-agente, configuraciones locales 
y binarios estén permanentemente actualizados en:
1. Repositorio Git (GitHub)
2. Respaldo en Google Drive (Copias espejo nocturnas/periódicas)
"""

import os
import time
import shutil
import subprocess
from pathlib import Path
from datetime import datetime

WORKSPACE_DIR = Path(r"D:\_______ia_local\Antigravity")
GIT_REPO_DIR = Path(r"D:\_______ia_local\Antigravity\mcp_rust_bridge")
GDRIVE_BACKUP_DIR = Path(r"I:\Mi unidad\Antigravity_Infra_Backup")

INTERVALO_SEGUNDOS = 3600  # Revisar cada 1 hora

def print_log(msg):
    print(f"[{datetime.now().strftime('%H:%M:%S')}] {msg}")

def ejecutar_git(comando):
    """Ejecuta un comando de git y devuelve el código de estado."""
    try:
        r = subprocess.run(
            ["git", "-C", str(GIT_REPO_DIR)] + comando,
            capture_output=True,
            text=True,
            check=False
        )
        return r.returncode, r.stdout.strip()
    except Exception as e:
        return -1, str(e)

def sync_git():
    """Verifica el estado del repositorio y hace auto-commit si hay cambios."""
    print_log("Comprobando cambios en Git...")
    rc, estado = ejecutar_git(["status", "--porcelain"])
    
    if rc != 0:
        print_log(f"Error comprobando Git: {estado}")
        return

    if not estado:
        print_log("Git: Árbol limpio. Nada que sincronizar.")
        return

    print_log(f"Git: Se detectaron {len(estado.splitlines())} archivos modificados.")
    
    # Añadir todo
    ejecutar_git(["add", "."])
    
    # Hacer commit
    mensaje = f"auto-sync: monitor de infraestructura ({datetime.now().strftime('%Y-%m-%d %H:%M')})"
    ejecutar_git(["commit", "-m", mensaje])
    
    # Push
    print_log("Git: Realizando push origin main...")
    rc_push, out_push = ejecutar_git(["push", "origin", "main"])
    
    if rc_push == 0:
        print_log("Git: Sincronización exitosa.")
    else:
        print_log(f"Git: Error en push -> {out_push}")

def sync_gdrive():
    """Hace una copia espejo del workspace a Google Drive (ignorando .git y compilados)."""
    if not GDRIVE_BACKUP_DIR.parent.exists():
        print_log("GDRIVE: Unidad no encontrada o no montada. Saltando backup.")
        return
        
    print_log("GDrive: Iniciando copia espejo...")
    try:
        # Excluir la pesada carpeta .git y los compilados de rust
        def ignore_patterns(d, files):
            return [f for f in files if f in ['.git', 'target', '__pycache__', 'node_modules']]

        if GDRIVE_BACKUP_DIR.exists():
            shutil.rmtree(GDRIVE_BACKUP_DIR, ignore_errors=True)
            
        shutil.copytree(WORKSPACE_DIR, GDRIVE_BACKUP_DIR, ignore=ignore_patterns, dirs_exist_ok=True)
        print_log(f"GDrive: Respaldo copiado exitosamente en {GDRIVE_BACKUP_DIR}.")
    except Exception as e:
        print_log(f"GDrive: Error durante el respaldo -> {e}")

def main():
    print_log("=== MONITOR MULTI-AGENTE INICIADO ===")
    print_log(f"Workspace: {WORKSPACE_DIR}")
    print_log("Presiona Ctrl+C para detener.")
    
    try:
        while True:
            sync_git()
            sync_gdrive()
            print_log(f"Durmiendo por {INTERVALO_SEGUNDOS / 60} minutos...")
            time.sleep(INTERVALO_SEGUNDOS)
    except KeyboardInterrupt:
        print_log("Monitor detenido por el usuario.")

if __name__ == "__main__":
    main()
