/**
 * These symbol exports mean you can link to the entire set of glcore
 * bindings with a simple `use glcore::*;`.
 */

/* Constant Exports */

#[cfg(GL_VERSION_1_1)]
pub use consts::GL_VERSION_1_1::*;

#[cfg(GL_VERSION_1_2)]
pub use consts::GL_VERSION_1_2::*;

#[cfg(GL_ARB_imaging)]
pub use consts::GL_ARB_imaging::*;

#[cfg(GL_VERSION_1_3)]
pub use consts::GL_VERSION_1_3::*;

#[cfg(GL_VERSION_1_4)]
pub use consts::GL_VERSION_1_4::*;

#[cfg(GL_VERSION_1_5)]
pub use consts::GL_VERSION_1_5::*;

#[cfg(GL_VERSION_2_0)]
pub use consts::GL_VERSION_2_0::*;

#[cfg(GL_VERSION_2_1)]
pub use consts::GL_VERSION_2_1::*;

#[cfg(GL_VERSION_3_0)]
pub use consts::GL_VERSION_3_0::*;

#[cfg(GL_VERSION_3_1)]
pub use consts::GL_VERSION_3_1::*;

#[cfg(GL_VERSION_3_2)]
pub use consts::GL_VERSION_3_2::*;

#[cfg(GL_VERSION_3_3)]
pub use consts::GL_VERSION_3_3::*;

#[cfg(GL_VERSION_4_0)]
pub use consts::GL_VERSION_4_0::*;

#[cfg(GL_VERSION_4_1)]
pub use consts::GL_VERSION_4_1::*;

#[cfg(GL_VERSION_4_2)]
pub use consts::GL_VERSION_4_2::*;

#[cfg(GL_VERSION_4_3)]
pub use consts::GL_VERSION_4_3::*;

#[cfg(GL_ARB_depth_buffer_float)]
pub use consts::GL_ARB_depth_buffer_float::*;

#[cfg(GL_ARB_framebuffer_object)]
pub use consts::GL_ARB_framebuffer_object::*;

#[cfg(GL_ARB_framebuffer_srgb)]
pub use consts::GL_ARB_framebuffer_srgb::*;

#[cfg(GL_ARB_half_float_vertex)]
pub use consts::GL_ARB_half_float_vertex::*;

#[cfg(GL_ARB_map_buffer_range)]
pub use consts::GL_ARB_map_buffer_range::*;

#[cfg(GL_ARB_texture_compression_rgtc)]
pub use consts::GL_ARB_texture_compression_rgtc::*;

#[cfg(GL_ARB_texture_rg)]
pub use consts::GL_ARB_texture_rg::*;

#[cfg(GL_ARB_vertex_array_object)]
pub use consts::GL_ARB_vertex_array_object::*;

#[cfg(GL_ARB_uniform_buffer_object)]
pub use consts::GL_ARB_uniform_buffer_object::*;

#[cfg(GL_ARB_copy_buffer)]
pub use consts::GL_ARB_copy_buffer::*;

#[cfg(GL_ARB_depth_clamp)]
pub use consts::GL_ARB_depth_clamp::*;

#[cfg(GL_ARB_draw_elements_base_vertex)]
pub use consts::GL_ARB_draw_elements_base_vertex::*;

#[cfg(GL_ARB_fragment_coord_conventions)]
pub use consts::GL_ARB_fragment_coord_conventions::*;

#[cfg(GL_ARB_provoking_vertex)]
pub use consts::GL_ARB_provoking_vertex::*;

#[cfg(GL_ARB_seamless_cube_map)]
pub use consts::GL_ARB_seamless_cube_map::*;

#[cfg(GL_ARB_sync)]
pub use consts::GL_ARB_sync::*;

#[cfg(GL_ARB_texture_multisample)]
pub use consts::GL_ARB_texture_multisample::*;

#[cfg(GL_ARB_vertex_array_bgra)]
pub use consts::GL_ARB_vertex_array_bgra::*;

#[cfg(GL_ARB_draw_buffers_blend)]
pub use consts::GL_ARB_draw_buffers_blend::*;

#[cfg(GL_ARB_sample_shading)]
pub use consts::GL_ARB_sample_shading::*;

#[cfg(GL_ARB_texture_cube_map_array)]
pub use consts::GL_ARB_texture_cube_map_array::*;

#[cfg(GL_ARB_texture_gather)]
pub use consts::GL_ARB_texture_gather::*;

#[cfg(GL_ARB_texture_query_lod)]
pub use consts::GL_ARB_texture_query_lod::*;

#[cfg(GL_ARB_shading_language_include)]
pub use consts::GL_ARB_shading_language_include::*;

#[cfg(GL_ARB_texture_compression_bptc)]
pub use consts::GL_ARB_texture_compression_bptc::*;

#[cfg(GL_ARB_blend_func_extended)]
pub use consts::GL_ARB_blend_func_extended::*;

#[cfg(GL_ARB_explicit_attrib_location)]
pub use consts::GL_ARB_explicit_attrib_location::*;

#[cfg(GL_ARB_occlusion_query2)]
pub use consts::GL_ARB_occlusion_query2::*;

#[cfg(GL_ARB_sampler_objects)]
pub use consts::GL_ARB_sampler_objects::*;

#[cfg(GL_ARB_shader_bit_encoding)]
pub use consts::GL_ARB_shader_bit_encoding::*;

#[cfg(GL_ARB_texture_rgb10_a2ui)]
pub use consts::GL_ARB_texture_rgb10_a2ui::*;

#[cfg(GL_ARB_texture_swizzle)]
pub use consts::GL_ARB_texture_swizzle::*;

#[cfg(GL_ARB_timer_query)]
pub use consts::GL_ARB_timer_query::*;

#[cfg(GL_ARB_vertex_type_2_10_10_10_rev)]
pub use consts::GL_ARB_vertex_type_2_10_10_10_rev::*;

#[cfg(GL_ARB_draw_indirect)]
pub use consts::GL_ARB_draw_indirect::*;

#[cfg(GL_ARB_gpu_shader5)]
pub use consts::GL_ARB_gpu_shader5::*;

#[cfg(GL_ARB_gpu_shader_fp64)]
pub use consts::GL_ARB_gpu_shader_fp64::*;

#[cfg(GL_ARB_shader_subroutine)]
pub use consts::GL_ARB_shader_subroutine::*;

#[cfg(GL_ARB_tessellation_shader)]
pub use consts::GL_ARB_tessellation_shader::*;

#[cfg(GL_ARB_texture_buffer_object_rgb32)]
pub use consts::GL_ARB_texture_buffer_object_rgb32::*;

#[cfg(GL_ARB_transform_feedback2)]
pub use consts::GL_ARB_transform_feedback2::*;

#[cfg(GL_ARB_transform_feedback3)]
pub use consts::GL_ARB_transform_feedback3::*;

#[cfg(GL_ARB_es2_compatibility)]
pub use consts::GL_ARB_es2_compatibility::*;

#[cfg(GL_ARB_get_program_binary)]
pub use consts::GL_ARB_get_program_binary::*;

#[cfg(GL_ARB_separate_shader_objects)]
pub use consts::GL_ARB_separate_shader_objects::*;

#[cfg(GL_ARB_shader_precision)]
pub use consts::GL_ARB_shader_precision::*;

#[cfg(GL_ARB_vertex_attrib_64bit)]
pub use consts::GL_ARB_vertex_attrib_64bit::*;

#[cfg(GL_ARB_viewport_array)]
pub use consts::GL_ARB_viewport_array::*;

#[cfg(GL_ARB_cl_event)]
pub use consts::GL_ARB_cl_event::*;

#[cfg(GL_ARB_debug_output)]
pub use consts::GL_ARB_debug_output::*;

#[cfg(GL_ARB_robustness)]
pub use consts::GL_ARB_robustness::*;

#[cfg(GL_ARB_shader_stencil_export)]
pub use consts::GL_ARB_shader_stencil_export::*;

#[cfg(GL_ARB_base_instance)]
pub use consts::GL_ARB_base_instance::*;

#[cfg(GL_ARB_shading_language_420pack)]
pub use consts::GL_ARB_shading_language_420pack::*;

#[cfg(GL_ARB_transform_feedback_instanced)]
pub use consts::GL_ARB_transform_feedback_instanced::*;

#[cfg(GL_ARB_compressed_texture_pixel_storage)]
pub use consts::GL_ARB_compressed_texture_pixel_storage::*;

#[cfg(GL_ARB_conservative_depth)]
pub use consts::GL_ARB_conservative_depth::*;

#[cfg(GL_ARB_internalformat_query)]
pub use consts::GL_ARB_internalformat_query::*;

#[cfg(GL_ARB_map_buffer_alignment)]
pub use consts::GL_ARB_map_buffer_alignment::*;

#[cfg(GL_ARB_shader_atomic_counters)]
pub use consts::GL_ARB_shader_atomic_counters::*;

#[cfg(GL_ARB_shader_image_load_store)]
pub use consts::GL_ARB_shader_image_load_store::*;

#[cfg(GL_ARB_shading_language_packing)]
pub use consts::GL_ARB_shading_language_packing::*;

#[cfg(GL_ARB_texture_storage)]
pub use consts::GL_ARB_texture_storage::*;

#[cfg(GL_KHR_texture_compression_astc_ldr)]
pub use consts::GL_KHR_texture_compression_astc_ldr::*;

#[cfg(GL_KHR_debug)]
pub use consts::GL_KHR_debug::*;

#[cfg(GL_ARB_arrays_of_arrays)]
pub use consts::GL_ARB_arrays_of_arrays::*;

#[cfg(GL_ARB_clear_buffer_object)]
pub use consts::GL_ARB_clear_buffer_object::*;

#[cfg(GL_ARB_compute_shader)]
pub use consts::GL_ARB_compute_shader::*;

#[cfg(GL_ARB_copy_image)]
pub use consts::GL_ARB_copy_image::*;

#[cfg(GL_ARB_texture_view)]
pub use consts::GL_ARB_texture_view::*;

#[cfg(GL_ARB_vertex_attrib_binding)]
pub use consts::GL_ARB_vertex_attrib_binding::*;

#[cfg(GL_ARB_robustness_isolation)]
pub use consts::GL_ARB_robustness_isolation::*;

#[cfg(GL_ARB_es3_compatibility)]
pub use consts::GL_ARB_es3_compatibility::*;

#[cfg(GL_ARB_explicit_uniform_location)]
pub use consts::GL_ARB_explicit_uniform_location::*;

#[cfg(GL_ARB_fragment_layer_viewport)]
pub use consts::GL_ARB_fragment_layer_viewport::*;

#[cfg(GL_ARB_framebuffer_no_attachments)]
pub use consts::GL_ARB_framebuffer_no_attachments::*;

#[cfg(GL_ARB_internalformat_query2)]
pub use consts::GL_ARB_internalformat_query2::*;

#[cfg(GL_ARB_invalidate_subdata)]
pub use consts::GL_ARB_invalidate_subdata::*;

#[cfg(GL_ARB_multi_draw_indirect)]
pub use consts::GL_ARB_multi_draw_indirect::*;

#[cfg(GL_ARB_program_interface_query)]
pub use consts::GL_ARB_program_interface_query::*;

#[cfg(GL_ARB_robust_buffer_access_behavior)]
pub use consts::GL_ARB_robust_buffer_access_behavior::*;

#[cfg(GL_ARB_shader_image_size)]
pub use consts::GL_ARB_shader_image_size::*;

#[cfg(GL_ARB_shader_storage_buffer_object)]
pub use consts::GL_ARB_shader_storage_buffer_object::*;

#[cfg(GL_ARB_stencil_texturing)]
pub use consts::GL_ARB_stencil_texturing::*;

#[cfg(GL_ARB_texture_buffer_range)]
pub use consts::GL_ARB_texture_buffer_range::*;

#[cfg(GL_ARB_texture_query_levels)]
pub use consts::GL_ARB_texture_query_levels::*;

#[cfg(GL_ARB_texture_storage_multisample)]
pub use consts::GL_ARB_texture_storage_multisample::*;


/* Function Exports */

#[cfg(GL_VERSION_1_0)]
pub use functions::GL_VERSION_1_0::*;

#[cfg(GL_VERSION_1_1)]
pub use functions::GL_VERSION_1_1::*;

#[cfg(GL_VERSION_1_2)]
pub use functions::GL_VERSION_1_2::*;

#[cfg(GL_VERSION_1_3)]
pub use functions::GL_VERSION_1_3::*;

#[cfg(GL_VERSION_1_4)]
pub use functions::GL_VERSION_1_4::*;

#[cfg(GL_VERSION_1_5)]
pub use functions::GL_VERSION_1_5::*;

#[cfg(GL_VERSION_2_0)]
pub use functions::GL_VERSION_2_0::*;

#[cfg(GL_VERSION_2_1)]
pub use functions::GL_VERSION_2_1::*;

#[cfg(GL_VERSION_3_0)]
pub use functions::GL_VERSION_3_0::*;

#[cfg(GL_VERSION_3_1)]
pub use functions::GL_VERSION_3_1::*;

#[cfg(GL_VERSION_3_2)]
pub use functions::GL_VERSION_3_2::*;

#[cfg(GL_VERSION_3_3)]
pub use functions::GL_VERSION_3_3::*;

#[cfg(GL_VERSION_4_0)]
pub use functions::GL_VERSION_4_0::*;

#[cfg(GL_VERSION_4_1)]
pub use functions::GL_VERSION_4_1::*;

#[cfg(GL_VERSION_4_2)]
pub use functions::GL_VERSION_4_2::*;

#[cfg(GL_VERSION_4_3)]
pub use functions::GL_VERSION_4_3::*;

#[cfg(GL_ARB_depth_buffer_float)]
pub use functions::GL_ARB_depth_buffer_float::*;

#[cfg(GL_ARB_framebuffer_object)]
pub use functions::GL_ARB_framebuffer_object::*;

#[cfg(GL_ARB_framebuffer_srgb)]
pub use functions::GL_ARB_framebuffer_srgb::*;

#[cfg(GL_ARB_half_float_vertex)]
pub use functions::GL_ARB_half_float_vertex::*;

#[cfg(GL_ARB_map_buffer_range)]
pub use functions::GL_ARB_map_buffer_range::*;

#[cfg(GL_ARB_texture_compression_rgtc)]
pub use functions::GL_ARB_texture_compression_rgtc::*;

#[cfg(GL_ARB_texture_rg)]
pub use functions::GL_ARB_texture_rg::*;

#[cfg(GL_ARB_vertex_array_object)]
pub use functions::GL_ARB_vertex_array_object::*;

#[cfg(GL_ARB_uniform_buffer_object)]
pub use functions::GL_ARB_uniform_buffer_object::*;

#[cfg(GL_ARB_copy_buffer)]
pub use functions::GL_ARB_copy_buffer::*;

#[cfg(GL_ARB_depth_clamp)]
pub use functions::GL_ARB_depth_clamp::*;

#[cfg(GL_ARB_draw_elements_base_vertex)]
pub use functions::GL_ARB_draw_elements_base_vertex::*;

#[cfg(GL_ARB_fragment_coord_conventions)]
pub use functions::GL_ARB_fragment_coord_conventions::*;

#[cfg(GL_ARB_provoking_vertex)]
pub use functions::GL_ARB_provoking_vertex::*;

#[cfg(GL_ARB_seamless_cube_map)]
pub use functions::GL_ARB_seamless_cube_map::*;

#[cfg(GL_ARB_sync)]
pub use functions::GL_ARB_sync::*;

#[cfg(GL_ARB_texture_multisample)]
pub use functions::GL_ARB_texture_multisample::*;

#[cfg(GL_ARB_vertex_array_bgra)]
pub use functions::GL_ARB_vertex_array_bgra::*;

#[cfg(GL_ARB_draw_buffers_blend)]
pub use functions::GL_ARB_draw_buffers_blend::*;

#[cfg(GL_ARB_sample_shading)]
pub use functions::GL_ARB_sample_shading::*;

#[cfg(GL_ARB_texture_cube_map_array)]
pub use functions::GL_ARB_texture_cube_map_array::*;

#[cfg(GL_ARB_texture_gather)]
pub use functions::GL_ARB_texture_gather::*;

#[cfg(GL_ARB_texture_query_lod)]
pub use functions::GL_ARB_texture_query_lod::*;

#[cfg(GL_ARB_shading_language_include)]
pub use functions::GL_ARB_shading_language_include::*;

#[cfg(GL_ARB_texture_compression_bptc)]
pub use functions::GL_ARB_texture_compression_bptc::*;

#[cfg(GL_ARB_blend_func_extended)]
pub use functions::GL_ARB_blend_func_extended::*;

#[cfg(GL_ARB_explicit_attrib_location)]
pub use functions::GL_ARB_explicit_attrib_location::*;

#[cfg(GL_ARB_occlusion_query2)]
pub use functions::GL_ARB_occlusion_query2::*;

#[cfg(GL_ARB_sampler_objects)]
pub use functions::GL_ARB_sampler_objects::*;

#[cfg(GL_ARB_shader_bit_encoding)]
pub use functions::GL_ARB_shader_bit_encoding::*;

#[cfg(GL_ARB_texture_rgb10_a2ui)]
pub use functions::GL_ARB_texture_rgb10_a2ui::*;

#[cfg(GL_ARB_texture_swizzle)]
pub use functions::GL_ARB_texture_swizzle::*;

#[cfg(GL_ARB_timer_query)]
pub use functions::GL_ARB_timer_query::*;

#[cfg(GL_ARB_vertex_type_2_10_10_10_rev)]
pub use functions::GL_ARB_vertex_type_2_10_10_10_rev::*;

#[cfg(GL_ARB_draw_indirect)]
pub use functions::GL_ARB_draw_indirect::*;

#[cfg(GL_ARB_gpu_shader5)]
pub use functions::GL_ARB_gpu_shader5::*;

#[cfg(GL_ARB_gpu_shader_fp64)]
pub use functions::GL_ARB_gpu_shader_fp64::*;

#[cfg(GL_ARB_shader_subroutine)]
pub use functions::GL_ARB_shader_subroutine::*;

#[cfg(GL_ARB_tessellation_shader)]
pub use functions::GL_ARB_tessellation_shader::*;

#[cfg(GL_ARB_texture_buffer_object_rgb32)]
pub use functions::GL_ARB_texture_buffer_object_rgb32::*;

#[cfg(GL_ARB_transform_feedback2)]
pub use functions::GL_ARB_transform_feedback2::*;

#[cfg(GL_ARB_transform_feedback3)]
pub use functions::GL_ARB_transform_feedback3::*;

#[cfg(GL_ARB_es2_compatibility)]
pub use functions::GL_ARB_es2_compatibility::*;

#[cfg(GL_ARB_get_program_binary)]
pub use functions::GL_ARB_get_program_binary::*;

#[cfg(GL_ARB_separate_shader_objects)]
pub use functions::GL_ARB_separate_shader_objects::*;

#[cfg(GL_ARB_vertex_attrib_64bit)]
pub use functions::GL_ARB_vertex_attrib_64bit::*;

#[cfg(GL_ARB_viewport_array)]
pub use functions::GL_ARB_viewport_array::*;

#[cfg(GL_ARB_cl_event)]
pub use functions::GL_ARB_cl_event::*;

#[cfg(GL_ARB_debug_output)]
pub use functions::GL_ARB_debug_output::*;

#[cfg(GL_ARB_robustness)]
pub use functions::GL_ARB_robustness::*;

#[cfg(GL_ARB_shader_stencil_export)]
pub use functions::GL_ARB_shader_stencil_export::*;

#[cfg(GL_ARB_base_instance)]
pub use functions::GL_ARB_base_instance::*;

#[cfg(GL_ARB_shading_language_420pack)]
pub use functions::GL_ARB_shading_language_420pack::*;

#[cfg(GL_ARB_transform_feedback_instanced)]
pub use functions::GL_ARB_transform_feedback_instanced::*;

#[cfg(GL_ARB_compressed_texture_pixel_storage)]
pub use functions::GL_ARB_compressed_texture_pixel_storage::*;

#[cfg(GL_ARB_conservative_depth)]
pub use functions::GL_ARB_conservative_depth::*;

#[cfg(GL_ARB_internalformat_query)]
pub use functions::GL_ARB_internalformat_query::*;

#[cfg(GL_ARB_map_buffer_alignment)]
pub use functions::GL_ARB_map_buffer_alignment::*;

#[cfg(GL_ARB_shader_atomic_counters)]
pub use functions::GL_ARB_shader_atomic_counters::*;

#[cfg(GL_ARB_shader_image_load_store)]
pub use functions::GL_ARB_shader_image_load_store::*;

#[cfg(GL_ARB_shading_language_packing)]
pub use functions::GL_ARB_shading_language_packing::*;

#[cfg(GL_ARB_texture_storage)]
pub use functions::GL_ARB_texture_storage::*;

#[cfg(GL_KHR_texture_compression_astc_ldr)]
pub use functions::GL_KHR_texture_compression_astc_ldr::*;

#[cfg(GL_KHR_debug)]
pub use functions::GL_KHR_debug::*;

#[cfg(GL_ARB_arrays_of_arrays)]
pub use functions::GL_ARB_arrays_of_arrays::*;

#[cfg(GL_ARB_clear_buffer_object)]
pub use functions::GL_ARB_clear_buffer_object::*;

#[cfg(GL_ARB_compute_shader)]
pub use functions::GL_ARB_compute_shader::*;

#[cfg(GL_ARB_copy_image)]
pub use functions::GL_ARB_copy_image::*;

#[cfg(GL_ARB_texture_view)]
pub use functions::GL_ARB_texture_view::*;

#[cfg(GL_ARB_vertex_attrib_binding)]
pub use functions::GL_ARB_vertex_attrib_binding::*;

#[cfg(GL_ARB_robustness_isolation)]
pub use functions::GL_ARB_robustness_isolation::*;

#[cfg(GL_ARB_es3_compatibility)]
pub use functions::GL_ARB_es3_compatibility::*;

#[cfg(GL_ARB_explicit_uniform_location)]
pub use functions::GL_ARB_explicit_uniform_location::*;

#[cfg(GL_ARB_fragment_layer_viewport)]
pub use functions::GL_ARB_fragment_layer_viewport::*;

#[cfg(GL_ARB_framebuffer_no_attachments)]
pub use functions::GL_ARB_framebuffer_no_attachments::*;

#[cfg(GL_ARB_internalformat_query2)]
pub use functions::GL_ARB_internalformat_query2::*;

#[cfg(GL_ARB_invalidate_subdata)]
pub use functions::GL_ARB_invalidate_subdata::*;

#[cfg(GL_ARB_multi_draw_indirect)]
pub use functions::GL_ARB_multi_draw_indirect::*;

#[cfg(GL_ARB_program_interface_query)]
pub use functions::GL_ARB_program_interface_query::*;

#[cfg(GL_ARB_robust_buffer_access_behavior)]
pub use functions::GL_ARB_robust_buffer_access_behavior::*;

#[cfg(GL_ARB_shader_image_size)]
pub use functions::GL_ARB_shader_image_size::*;

#[cfg(GL_ARB_shader_storage_buffer_object)]
pub use functions::GL_ARB_shader_storage_buffer_object::*;

#[cfg(GL_ARB_stencil_texturing)]
pub use functions::GL_ARB_stencil_texturing::*;

#[cfg(GL_ARB_texture_buffer_range)]
pub use functions::GL_ARB_texture_buffer_range::*;

#[cfg(GL_ARB_texture_query_levels)]
pub use functions::GL_ARB_texture_query_levels::*;

#[cfg(GL_ARB_texture_storage_multisample)]
pub use functions::GL_ARB_texture_storage_multisample::*;


/* Type Exports */

#[cfg(GL_VERSION_1_0)]
pub use types::GL_VERSION_1_0::*;

#[cfg(GL_VERSION_2_0)]
pub use types::GL_VERSION_2_0::*;

#[cfg(GL_VERSION_1_5)]
pub use types::GL_VERSION_1_5::*;

#[cfg(GL_ARB_sync)]
pub use types::GL_ARB_sync::*;

#[cfg(GL_ARB_vertex_buffer_object)]
pub use types::GL_ARB_vertex_buffer_object::*;

#[cfg(GL_ARB_shader_objects)]
pub use types::GL_ARB_shader_objects::*;

#[cfg(GL_ARB_half_float_pixel)]
pub use types::GL_ARB_half_float_pixel::*;

#[cfg(GL_NV_half_float)]
pub use types::GL_NV_half_float::*;

#[cfg(GL_EXT_timer_query)]
pub use types::GL_EXT_timer_query::*;

#[cfg(GL_ARB_cl_event)]
pub use types::GL_ARB_cl_event::*;

#[cfg(GL_ARB_debug_output)]
pub use types::GL_ARB_debug_output::*;

#[cfg(GL_AMD_debug_output)]
pub use types::GL_AMD_debug_output::*;

#[cfg(GL_KHR_debug)]
pub use types::GL_KHR_debug::*;

#[cfg(GL_NV_vdpau_interop)]
pub use types::GL_NV_vdpau_interop::*;