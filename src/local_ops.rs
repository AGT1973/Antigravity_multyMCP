use std::fs;
use std::path::Path;
use std::process::Command;
use chrono::Local;
use serde_json::{json, Value};
use uuid::Uuid;
use walkdir::WalkDir;

pub fn ejecutar(operacion: &str, args: Value) -> Result<String, String> {
    match operacion {
        // --- LECTURA ---
        "leer_txt" | "leer_md" | "leer_json" => {
            let ruta = args.get("ruta").and_then(|v| v.as_str()).ok_or("Falta 'ruta'")?;
            let mut content = fs::read_to_string(ruta).map_err(|e| e.to_string())?;
            if operacion == "leer_txt" {
                let max_chars = 800 * 100; // Limit roughly 800 lines
                if content.len() > max_chars {
                    content.truncate(max_chars);
                    content.push_str("\n...[truncado]");
                }
            }
            Ok(content)
        }
        "leer_csv" => {
            let ruta = args.get("ruta").and_then(|v| v.as_str()).ok_or("Falta 'ruta'")?;
            let content = fs::read_to_string(ruta).map_err(|e| e.to_string())?;
            let limit = args.get("filas").and_then(|v| v.as_u64()).unwrap_or(20) as usize;
            let lines: Vec<&str> = content.lines().take(limit).collect();
            Ok(lines.join("\n"))
        }

        // --- ESCRITURA ---
        "guardar_archivo" | "guardar_json" => {
            let ruta = args.get("ruta").and_then(|v| v.as_str()).ok_or("Falta 'ruta'")?;
            // Para json, args.get("datos") puede ser String o Object. Si es object, convertir a string.
            let contenido = if operacion == "guardar_json" {
                if let Some(datos_str) = args.get("datos").and_then(|v| v.as_str()) {
                    datos_str.to_string()
                } else {
                    args.get("datos").map(|v| v.to_string()).ok_or("Falta 'datos'")?
                }
            } else {
                args.get("contenido").and_then(|v| v.as_str()).ok_or("Falta 'contenido'")?.to_string()
            };
            if let Some(parent) = Path::new(ruta).parent() {
                let _ = fs::create_dir_all(parent);
            }
            fs::write(ruta, contenido).map_err(|e| e.to_string())?;
            Ok(format!("Guardado en {}", ruta))
        }
        "agregar_linea" => {
            let ruta = args.get("ruta").and_then(|v| v.as_str()).ok_or("Falta 'ruta'")?;
            let texto = args.get("texto").and_then(|v| v.as_str()).ok_or("Falta 'texto'")?;
            use std::io::Write;
            let mut file = fs::OpenOptions::new().create(true).append(true).open(ruta).map_err(|e| e.to_string())?;
            writeln!(file, "{}", texto).map_err(|e| e.to_string())?;
            Ok(format!("Linea agregada a {}", ruta))
        }
        "reemplazar_texto" => {
            let ruta = args.get("ruta").and_then(|v| v.as_str()).ok_or("Falta 'ruta'")?;
            let buscar = args.get("buscar").and_then(|v| v.as_str()).ok_or("Falta 'buscar'")?;
            let reemplazar = args.get("reemplazar").and_then(|v| v.as_str()).ok_or("Falta 'reemplazar'")?;
            let original = fs::read_to_string(ruta).map_err(|e| e.to_string())?;
            let count = original.matches(buscar).count();
            let nuevo = original.replace(buscar, reemplazar);
            fs::write(ruta, nuevo).map_err(|e| e.to_string())?;
            Ok(format!("Reemplazadas {} ocurrencias", count))
        }

        // --- MANIPULACION ---
        "eliminar_archivo" => {
            let ruta = args.get("ruta").and_then(|v| v.as_str()).ok_or("Falta 'ruta'")?;
            fs::remove_file(ruta).map_err(|e| e.to_string())?;
            Ok("Eliminado".into())
        }
        "mover_archivo" => {
            let origen = args.get("origen").and_then(|v| v.as_str()).ok_or("Falta 'origen'")?;
            let destino = args.get("destino").and_then(|v| v.as_str()).ok_or("Falta 'destino'")?;
            if let Some(parent) = Path::new(destino).parent() { let _ = fs::create_dir_all(parent); }
            fs::rename(origen, destino).map_err(|e| e.to_string())?;
            Ok("Movido".into())
        }
        "copiar_archivo" => {
            let origen = args.get("origen").and_then(|v| v.as_str()).ok_or("Falta 'origen'")?;
            let destino = args.get("destino").and_then(|v| v.as_str()).ok_or("Falta 'destino'")?;
            if let Some(parent) = Path::new(destino).parent() { let _ = fs::create_dir_all(parent); }
            fs::copy(origen, destino).map_err(|e| e.to_string())?;
            Ok("Copiado".into())
        }
        "info_archivo" => {
            let ruta = args.get("ruta").and_then(|v| v.as_str()).ok_or("Falta 'ruta'")?;
            let metadata = fs::metadata(ruta).map_err(|e| e.to_string())?;
            Ok(json!({ "tamano_bytes": metadata.len(), "es_directorio": metadata.is_dir() }).to_string())
        }

        // --- DIRECTORIOS ---
        "listar_archivos" => {
            let dir = args.get("directorio").and_then(|v| v.as_str()).ok_or("Falta 'directorio'")?;
            let mut res = vec![];
            for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()).take(200) {
                if entry.file_type().is_file() {
                    res.push(entry.path().display().to_string());
                }
            }
            Ok(json!({ "archivos": res }).to_string())
        }
        "listar_directorio" => {
            let dir = args.get("directorio").and_then(|v| v.as_str()).ok_or("Falta 'directorio'")?;
            let mut res = vec![];
            for entry in WalkDir::new(dir).min_depth(1).max_depth(2).into_iter().filter_map(|e| e.ok()) {
                res.push(format!("{}{}", entry.path().display(), if entry.path().is_dir() { "/" } else { "" }));
            }
            Ok(res.join("\n"))
        }
        "crear_directorio" => {
            let ruta = args.get("ruta").and_then(|v| v.as_str()).ok_or("Falta 'ruta'")?;
            fs::create_dir_all(ruta).map_err(|e| e.to_string())?;
            Ok("Creado".into())
        }
        "eliminar_directorio" => {
            let ruta = args.get("ruta").and_then(|v| v.as_str()).ok_or("Falta 'ruta'")?;
            fs::remove_dir_all(ruta).map_err(|e| e.to_string())?;
            Ok("Eliminado".into())
        }

        // --- GIT & CMD ---
        "git_status" => {
            let dir = args.get("directorio").and_then(|v| v.as_str()).ok_or("Falta 'directorio'")?;
            let out = Command::new("git").args(["-C", dir, "status", "--short"]).output().map_err(|e| e.to_string())?;
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        }
        "git_log" => {
            let dir = args.get("directorio").and_then(|v| v.as_str()).ok_or("Falta 'directorio'")?;
            let out = Command::new("git").args(["-C", dir, "log", "-10", "--oneline"]).output().map_err(|e| e.to_string())?;
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        }
        "git_diff" => {
            let dir = args.get("directorio").and_then(|v| v.as_str()).ok_or("Falta 'directorio'")?;
            let out = Command::new("git").args(["-C", dir, "diff"]).output().map_err(|e| e.to_string())?;
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        }
        "ejecutar_cmd" => {
            let cmd = args.get("comando").and_then(|v| v.as_str()).ok_or("Falta 'comando'")?;
            let cwd = args.get("directorio").and_then(|v| v.as_str()).unwrap_or(".");
            // Simple windows shell execution
            let out = Command::new("cmd").args(["/C", cmd]).current_dir(cwd).output().map_err(|e| e.to_string())?;
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            Ok(json!({"rc": out.status.code(), "stdout": stdout, "stderr": stderr}).to_string())
        }

        // --- MISC ---
        "tiempo_actual" => {
            Ok(json!({"iso": Local::now().to_rfc3339()}).to_string())
        }
        "uuid_gen" => {
            Ok(Uuid::new_v4().to_string())
        }

        _ => Err(format!("Operacion desconocida: {}", operacion))
    }
}
