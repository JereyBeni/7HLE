# 7HLE
# 7HLE 🚀

**7HLE** es una capa de compatibilidad HLE (High-Level Emulation) nativa desarrollada en **Rust** (sin dependencias pesadas como Electron) para ejecutar aplicaciones clásicas de Windows 7 en sistemas modernos mediante la intercepción de llamadas a la API de Win32 y un entorno visual inspirado en el clásico escritorio Aero de Windows 7.

## Características

- 🪟 **Interfaz nativa en Rust** con `iced` (liviana, rápida y sin bloatware).
- 📂 **Cargador PE integrado** para analizar ejecutables de Windows al vuelo.
- ⚙️ **Stubs HLE** para interceptar funciones críticas de `kernel32.dll`.
- 💻 **Multiplataforma** y liviano frente a emuladores pesados de hardware (como PCem).