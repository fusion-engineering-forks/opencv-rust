#![allow(unused_parens)]
//! # DNN used for super resolution
//! 
//! This module contains functionality for upscaling an image via convolutional neural networks.
//! The following four models are implemented:
//! 
//! - EDSR <https://arxiv.org/abs/1707.02921>
//! - ESPCN <https://arxiv.org/abs/1609.05158>
//! - FSRCNN <https://arxiv.org/abs/1608.00367>
//! - LapSRN <https://arxiv.org/abs/1710.01992>
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DnnSuperResImplTrait };
}

/// A class to upscale images via convolutional neural networks.
/// The following four models are implemented:
/// 
/// - edsr
/// - espcn
/// - fsrcnn
/// - lapsrn
pub trait DnnSuperResImplTrait {
	fn as_raw_DnnSuperResImpl(&self) -> *const c_void;
	fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void;

	/// Read the model from the given path
	/// ## Parameters
	/// * path: Path to the model file.
	fn read_model(&mut self, path: &str) -> Result<()> {
		string_arg!(path);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_readModel_const_StringX(self.as_raw_mut_DnnSuperResImpl(), path.as_ptr()) }.into_result()
	}
	
	/// Read the model from the given path
	/// ## Parameters
	/// * weights: Path to the model weights file.
	/// * definition: Path to the model definition file.
	fn read_model_1(&mut self, weights: &str, definition: &str) -> Result<()> {
		string_arg!(weights);
		string_arg!(definition);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_readModel_const_StringX_const_StringX(self.as_raw_mut_DnnSuperResImpl(), weights.as_ptr(), definition.as_ptr()) }.into_result()
	}
	
	/// Set desired model
	/// ## Parameters
	/// * algo: String containing one of the desired models:
	///    - __edsr__
	///    - __espcn__
	///    - __fsrcnn__
	///    - __lapsrn__
	/// * scale: Integer specifying the upscale factor
	fn set_model(&mut self, algo: &str, scale: i32) -> Result<()> {
		string_arg!(algo);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_setModel_const_StringX_int(self.as_raw_mut_DnnSuperResImpl(), algo.as_ptr(), scale) }.into_result()
	}
	
	/// Upsample via neural network
	/// ## Parameters
	/// * img: Image to upscale
	/// * result: Destination upscaled image
	fn upsample(&mut self, img: &dyn core::ToInputArray, result: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(img);
		output_array_arg!(result);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_upsample_const__InputArrayX_const__OutputArrayX(self.as_raw_mut_DnnSuperResImpl(), img.as_raw__InputArray(), result.as_raw__OutputArray()) }.into_result()
	}
	
	/// Upsample via neural network of multiple outputs
	/// ## Parameters
	/// * img: Image to upscale
	/// * imgs_new: Destination upscaled images
	/// * scale_factors: Scaling factors of the output nodes
	/// * node_names: Names of the output nodes in the neural network
	fn upsample_multioutput(&mut self, img: &dyn core::ToInputArray, imgs_new: &mut core::Vector::<core::Mat>, scale_factors: &core::Vector::<i32>, node_names: &core::Vector::<String>) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_upsampleMultioutput_const__InputArrayX_vector_Mat_X_const_vector_int_X_const_vector_String_X(self.as_raw_mut_DnnSuperResImpl(), img.as_raw__InputArray(), imgs_new.as_raw_mut_VectorOfMat(), scale_factors.as_raw_VectorOfi32(), node_names.as_raw_VectorOfString()) }.into_result()
	}
	
	/// Returns the scale factor of the model:
	/// ## Returns
	/// Current scale factor.
	fn get_scale(&mut self) -> Result<i32> {
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_getScale(self.as_raw_mut_DnnSuperResImpl()) }.into_result()
	}
	
	/// Returns the scale factor of the model:
	/// ## Returns
	/// Current algorithm.
	fn get_algorithm(&mut self) -> Result<String> {
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_getAlgorithm(self.as_raw_mut_DnnSuperResImpl()) }.into_result().map(|s| unsafe { crate::templ::receive_string(s as *mut String) })
	}
	
}

/// A class to upscale images via convolutional neural networks.
/// The following four models are implemented:
/// 
/// - edsr
/// - espcn
/// - fsrcnn
/// - lapsrn
pub struct DnnSuperResImpl {
	ptr: *mut c_void
}

boxed_ptr! { DnnSuperResImpl }

impl Drop for DnnSuperResImpl {
	fn drop(&mut self) {
		extern "C" { fn cv_DnnSuperResImpl_delete(instance: *mut c_void); }
		unsafe { cv_DnnSuperResImpl_delete(self.as_raw_mut_DnnSuperResImpl()) };
	}
}

impl DnnSuperResImpl {
	pub fn as_raw_DnnSuperResImpl(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void { self.as_raw_mut() }
}

unsafe impl Send for DnnSuperResImpl {}

impl crate::dnn_superres::DnnSuperResImplTrait for DnnSuperResImpl {
	fn as_raw_DnnSuperResImpl(&self) -> *const c_void { self.as_raw() }
	fn as_raw_mut_DnnSuperResImpl(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl DnnSuperResImpl {
	/// Empty constructor for python
	pub fn create() -> Result<core::Ptr::<crate::dnn_superres::DnnSuperResImpl>> {
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_create() }.into_result().map(|ptr| unsafe { core::Ptr::<crate::dnn_superres::DnnSuperResImpl>::from_raw(ptr) })
	}
	
	pub fn default() -> Result<crate::dnn_superres::DnnSuperResImpl> {
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl() }.into_result().map(|ptr| unsafe { crate::dnn_superres::DnnSuperResImpl::from_raw(ptr) })
	}
	
	/// Constructor which immediately sets the desired model
	/// ## Parameters
	/// * algo: String containing one of the desired models:
	///    - __edsr__
	///    - __espcn__
	///    - __fsrcnn__
	///    - __lapsrn__
	/// * scale: Integer specifying the upscale factor
	pub fn new(algo: &str, scale: i32) -> Result<crate::dnn_superres::DnnSuperResImpl> {
		string_arg!(algo);
		unsafe { sys::cv_dnn_superres_DnnSuperResImpl_DnnSuperResImpl_const_StringX_int(algo.as_ptr(), scale) }.into_result().map(|ptr| unsafe { crate::dnn_superres::DnnSuperResImpl::from_raw(ptr) })
	}
	
}
