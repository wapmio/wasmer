//! This module mainly outputs the `Compiler` trait that custom
//! compilers will need to implement.

use crate::error::CompileError;
use crate::function::Compilation;
use crate::lib::std::boxed::Box;
use crate::lib::std::sync::Arc;
use crate::module::CompileModuleInfo;
use crate::target::Target;
use crate::translator::FunctionMiddlewareGenerator;
use crate::FunctionBodyData;
use crate::ModuleTranslationState;
use wasm_common::entity::PrimaryMap;
use wasm_common::{Features, LocalFunctionIndex};
use wasmparser::{validate, OperatorValidatorConfig, ValidatingParserConfig};

/// The compiler configuration options.
pub trait CompilerConfig {
    /// Enable Position Independent Code (PIC).
    ///
    /// This is required for shared object generation (Native Engine),
    /// but will make the JIT Engine to fail, since PIC is not yet
    /// supported in the JIT linking phase.
    fn enable_pic(&mut self) {
        // By default we do nothing, each backend will need to customize this
        // in case they do something special for emitting PIC code.
    }

    /// Enable compiler IR verification.
    ///
    /// For compilers capable of doing so, this enables internal consistency
    /// checking.
    fn enable_verifier(&mut self) {
        // By default we do nothing, each backend will need to customize this
        // in case they create an IR that they can verify.
    }

    /// Gets the custom compiler config
    fn compiler(&self) -> Box<dyn Compiler + Send>;

    /// Gets the default features for this compiler in the given target
    fn default_features_for_target(&self, _target: &Target) -> Features {
        Features::default()
    }

    /// Pushes a middleware onto the back of the middleware chain.
    fn push_middleware(&mut self, middleware: Arc<dyn FunctionMiddlewareGenerator>);
}

/// An implementation of a Compiler from parsed WebAssembly module to Compiled native code.
pub trait Compiler {
    /// Validates a module.
    ///
    /// It returns the a succesful Result in case is valid, `CompileError` in case is not.
    fn validate_module<'data>(
        &self,
        features: &Features,
        data: &'data [u8],
    ) -> Result<(), CompileError> {
        let config = ValidatingParserConfig {
            operator_config: OperatorValidatorConfig {
                enable_threads: features.threads,
                enable_reference_types: features.reference_types,
                enable_bulk_memory: features.bulk_memory,
                enable_tail_call: false,
                enable_simd: features.simd,
                enable_multi_value: features.multi_value,
            },
        };
        validate(data, Some(config)).map_err(|e| CompileError::Validate(format!("{}", e)))
    }

    /// Compiles a parsed module.
    ///
    /// It returns the [`Compilation`] or a [`CompileError`].
    fn compile_module<'data, 'module>(
        &self,
        target: &Target,
        module: &'module CompileModuleInfo,
        module_translation: &ModuleTranslationState,
        // The list of function bodies
        function_body_inputs: PrimaryMap<LocalFunctionIndex, FunctionBodyData<'data>>,
    ) -> Result<Compilation, CompileError>;
}