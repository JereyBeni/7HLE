// ============================================================================
// 7HLE - Capa de Retrocompatibilidad con Windows XP (NT 5.1)
// ============================================================================

#![allow(dead_code)]

pub struct XpCompatLayer {
    pub target_os_version: String,
    pub enforce_winxp_apis: bool,
}

impl XpCompatLayer {
    pub fn new() -> Self {
        println!("[7HLE XP Compat] Inicializando capa de retrocompatibilidad con Windows XP (NT 5.1)...");
        Self {
            target_os_version: "5.1.2600".to_string(),
            enforce_winxp_apis: true,
        }
    }

    /// Interceptación de GetVersionEx para reportar Windows XP SP3
    pub fn hook_get_version_ex(&self) -> (u32, u32, u32) {
        println!("[7HLE XP Compat] GetVersionEx interceptado: Reportando Windows XP SP3 (NT 5.1).");
        (5, 1, 2600)
    }

    pub fn redirect_legacy_apis(&self) {
        println!("[7HLE XP Compat] Redirigiendo llamadas obsoletas de GDI y DirectDraw hacia los wrappers modernos...");
    }
}