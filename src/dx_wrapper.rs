// ============================================================================
// 7HLE - Subsistema Extendido de Emulación/Hooks para DirectX (9, 10 y 11)
// NOTA: Módulo independiente. Aún no conectado al loop principal.
// ============================================================================

#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectXVersion {
    DirectX9,
    DirectX10,
    DirectX11,
}

#[derive(Debug, Clone)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub format: String,
}

#[derive(Debug, Clone)]
pub struct SwapChainConfig {
    pub buffer_count: u32,
    pub display_mode: DisplayMode,
    pub windowed: bool,
    pub sample_count: u32,
}

// ----------------------------------------------------------------------------
// DIRECTX 9 LAYER (d3d9.dll / IDirect3D9 & IDirect3DDevice9)
// ----------------------------------------------------------------------------
pub struct Direct3D9Device {
    pub adapter_index: u32,
    pub device_type: String,
    pub config: SwapChainConfig,
    pub vram_allocated_mb: usize,
}

impl Direct3D9Device {
    pub fn new(config: SwapChainConfig) -> Self {
        println!("[7HLE DX9] IDirect3D9::CreateDevice ejecutado.");
        println!("[7HLE DX9] Configurando resolución: {}x{} (Windowed: {})", config.display_mode.width, config.display_mode.height, config.windowed);
        Self {
            adapter_index: 0,
            device_type: "D3DDEVTYPE_HAL".to_string(),
            config,
            vram_allocated_mb: 256,
        }
    }

    pub fn set_render_state(&self, state: &str, value: u32) {
        println!("[7HLE DX9] SetRenderState -> State: {}, Value: {}", state, value);
    }

    pub fn set_texture(&self, stage: u32, texture_name: &str) {
        println!("[7HLE DX9] SetTexture -> Stage: {}, Asset: {}", stage, texture_name);
    }

    pub fn draw_primitive(&self, primitive_type: &str, start_vertex: u32, primitive_count: u32) {
        println!("[7HLE DX9] DrawPrimitive -> Type: {}, Start: {}, Count: {}", primitive_type, start_vertex, primitive_count);
    }

    pub fn present(&self) {
        println!("[7HLE DX9] IDirect3DDevice9::Present -> Volcando Framebuffer a pantalla.");
    }
}

// ----------------------------------------------------------------------------
// DIRECTX 10 LAYER (d3d10.dll / ID3D10Device)
// ----------------------------------------------------------------------------
pub struct Direct3D10Device {
    pub feature_level: String,
    pub pipeline_bound: bool,
    pub active_geometry_shader: bool,
}

impl Direct3D10Device {
    pub fn new() -> Self {
        println!("[7HLE DX10] D3D10CreateDeviceAndSwapChain interceptado.");
        Self {
            feature_level: "10_0".to_string(),
            pipeline_bound: true,
            active_geometry_shader: false,
        }
    }

    pub fn create_buffer(&self, size_bytes: usize, usage: &str) {
        println!("[7HLE DX10] D3D10_BUFFER_DESC -> Asignando {} bytes (Usage: {})", size_bytes, usage);
    }

    pub fn set_input_layout(&self, layout_name: &str) {
        println!("[7HLE DX10] IASetInputLayout -> Layout: {}", layout_name);
    }

    pub fn draw_indexed(&self, index_count: u32, start_index_location: u32, base_vertex_location: i32) {
        println!("[7HLE DX10] DrawIndexed -> Count: {}, StartIdx: {}, BaseVertex: {}", index_count, start_index_location, base_vertex_location);
    }
}

// ----------------------------------------------------------------------------
// DIRECTX 11 LAYER (d3d11.dll / ID3D11Device & ID3D11DeviceContext)
// ----------------------------------------------------------------------------
pub struct Direct3D11Pipeline {
    pub feature_level: String,
    pub compute_shaders_enabled: bool,
    pub tessellation_enabled: bool,
}

impl Direct3D11Pipeline {
    pub fn new() -> Self {
        println!("[7HLE DX11] D3D11CreateDeviceAndSwapChain inicializado.");
        println!("[7HLE DX11] Feature Level fijado en 11_0 (Windows 7 / Direct3D 11 SDK).");
        Self {
            feature_level: "11_0".to_string(),
            compute_shaders_enabled: true,
            tessellation_enabled: true,
        }
    }

    pub fn vs_set_shader(&self, shader_name: &str) {
        println!("[7HLE DX11 Context] VSSetShader -> Vertex Shader: {}", shader_name);
    }

    pub fn ps_set_shader(&self, shader_name: &str) {
        println!("[7HLE DX11 Context] PSSetShader -> Pixel Shader: {}", shader_name);
    }

    pub fn dispatch_compute(&self, thread_group_count_x: u32, thread_group_count_y: u32, thread_group_count_z: u32) {
        println!("[7HLE DX11 Context] Dispatch (Compute Shader) -> X: {}, Y: {}, Z: {}", thread_group_count_x, thread_group_count_y, thread_group_count_z);
    }

    pub fn swapchain_present(&self, sync_interval: u32) {
        println!("[7HLE DX11 DXGI] IDXGISwapChain::Present -> SyncInterval: {}", sync_interval);
    }
}

// ----------------------------------------------------------------------------
// MANAGER GENERAL DE DIRECTX
// ----------------------------------------------------------------------------
pub struct DirectXManager {
    pub active_version: Option<DirectXVersion>,
    pub dx9: Option<Direct3D9Device>,
    pub dx10: Option<Direct3D10Device>,
    pub dx11: Option<Direct3D11Pipeline>,
}

impl DirectXManager {
    pub fn new() -> Self {
        Self {
            active_version: None,
            dx9: None,
            dx10: None,
            dx11: None,
        }
    }

    pub fn init_version(&mut self, version: DirectXVersion) {
        self.active_version = Some(version);
        let default_config = SwapChainConfig {
            buffer_count: 2,
            display_mode: DisplayMode {
                width: 1280,
                height: 720,
                refresh_rate: 60,
                format: "DXGI_FORMAT_R8G8B8A8_UNORM".to_string(),
            },
            windowed: true,
            sample_count: 1,
        };

        match version {
            DirectXVersion::DirectX9 => {
                self.dx9 = Some(Direct3D9Device::new(default_config));
            }
            DirectXVersion::DirectX10 => {
                self.dx10 = Some(Direct3D10Device::new());
            }
            DirectXVersion::DirectX11 => {
                self.dx11 = Some(Direct3D11Pipeline::new());
            }
        }
    }
}