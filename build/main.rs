#![allow(unstable)]

extern crate gl_generator;
extern crate khronos_api;

use std::os;
use std::old_io::File;

mod textures;

fn main() {
    let dest = Path::new(os::getenv("OUT_DIR").unwrap());

    textures::build_texture_file(&mut File::create(&dest.join("textures.rs")).unwrap());


    let mut gl_bindings = File::create(&dest.join("gl_bindings.rs")).unwrap();
    gl_generator::generate_bindings(gl_generator::StructGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    khronos_api::GL_XML,
                                    vec![
                                        "GL_EXT_direct_state_access".to_string(),
                                        "GL_EXT_framebuffer_object".to_string(),
                                        "GL_EXT_framebuffer_blit".to_string(),
                                        "GL_NVX_gpu_memory_info".to_string(),
                                        "GL_ATI_meminfo".to_string(),
                                        "GL_EXT_texture_filter_anisotropic".to_string(),
                                        "GL_ARB_buffer_storage".to_string(),
                                        "GL_APPLE_vertex_array_object".to_string(),
                                        "GL_ARB_vertex_buffer_object".to_string(),
                                        "GL_ARB_shader_objects".to_string(),
                                        "GL_ARB_vertex_shader".to_string(),
                                        "GL_ARB_texture_rg".to_string(),
                                        "GL_EXT_texture_integer".to_string(),
                                        "GL_ARB_texture_rgb10_a2ui".to_string(),
                                        "GL_ARB_texture_float".to_string(),
                                        "GL_EXT_packed_depth_stencil".to_string(),
                                        "GL_ARB_debug_output".to_string(),
                                    ],
                                    "4.5", "compatibility", &mut gl_bindings).unwrap();
}
