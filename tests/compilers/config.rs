use std::sync::Arc;
use wasmer::{CompilerConfig, Engine as WasmerEngine, Features, ModuleMiddleware, Store};

#[derive(Clone, Debug, PartialEq)]
pub enum Compiler {
    LLVM,
    Cranelift,
    Singlepass,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Engine {
    SharedObject,
    Universal,
}

#[derive(Clone)]
pub struct Config {
    pub compiler: Compiler,
    pub engine: Engine,
    pub features: Option<Features>,
    pub middlewares: Vec<Arc<dyn ModuleMiddleware>>,
    pub canonicalize_nans: bool,
}

impl Config {
    pub fn new(engine: Engine, compiler: Compiler) -> Self {
        Self {
            compiler,
            engine,
            features: None,
            canonicalize_nans: false,
            middlewares: vec![],
        }
    }

    pub fn set_middlewares(&mut self, middlewares: Vec<Arc<dyn ModuleMiddleware>>) {
        self.middlewares = middlewares;
    }

    pub fn set_features(&mut self, features: Features) {
        self.features = Some(features);
    }

    pub fn set_nan_canonicalization(&mut self, canonicalize_nans: bool) {
        self.canonicalize_nans = canonicalize_nans;
    }

    pub fn store(&self) -> Store {
        let compiler_config = self.compiler_config(self.canonicalize_nans);
        let engine = self.engine(compiler_config);
        Store::new(&*engine)
    }

    pub fn headless_store(&self) -> Store {
        let engine = self.engine_headless();
        Store::new(&*engine)
    }

    pub fn engine(&self, compiler_config: Box<dyn CompilerConfig>) -> Box<dyn WasmerEngine> {
        #[cfg(not(feature = "engine"))]
        compile_error!("Plese enable at least one engine via the features");
        match &self.engine {
            #[cfg(feature = "shared-object")]
            Engine::SharedObject => {
                let mut engine = wasmer_engine_shared_object::SharedObject::new(compiler_config);
                if let Some(ref features) = self.features {
                    engine = engine.features(features.clone())
                }
                Box::new(engine.engine())
            }
            #[cfg(feature = "universal")]
            Engine::Universal => {
                let mut engine = wasmer_engine_universal::Universal::new(compiler_config);
                if let Some(ref features) = self.features {
                    engine = engine.features(features.clone())
                }
                Box::new(engine.engine())
            }
            #[allow(dead_code)]
            engine => panic!(
                "The {:?} Engine is not enabled. Please enable it using the features",
                engine
            ),
        }
    }

    pub fn engine_headless(&self) -> Box<dyn WasmerEngine> {
        match &self.engine {
            #[cfg(feature = "shared-object")]
            Engine::SharedObject => {
                Box::new(wasmer_engine_shared_object::SharedObject::headless().engine())
            }
            #[cfg(feature = "universal")]
            Engine::Universal => Box::new(wasmer_engine_universal::Universal::headless().engine()),
            #[allow(dead_code)]
            engine => panic!(
                "The {:?} Engine is not enabled. Please enable it using the features",
                engine
            ),
        }
    }

    pub fn compiler_config(&self, canonicalize_nans: bool) -> Box<dyn CompilerConfig> {
        match &self.compiler {
            #[cfg(feature = "cranelift")]
            Compiler::Cranelift => {
                let mut compiler = wasmer_compiler_cranelift::Cranelift::new();
                compiler.canonicalize_nans(canonicalize_nans);
                compiler.enable_verifier();
                self.add_middlewares(&mut compiler);
                Box::new(compiler)
            }
            #[cfg(feature = "llvm")]
            Compiler::LLVM => {
                let mut compiler = wasmer_compiler_llvm::LLVM::new();
                compiler.canonicalize_nans(canonicalize_nans);
                compiler.enable_verifier();
                self.add_middlewares(&mut compiler);
                Box::new(compiler)
            }
            #[cfg(feature = "singlepass")]
            Compiler::Singlepass => {
                let mut compiler = wasmer_compiler_singlepass::Singlepass::new();
                compiler.canonicalize_nans(canonicalize_nans);
                compiler.enable_verifier();
                self.add_middlewares(&mut compiler);
                Box::new(compiler)
            }
            #[allow(dead_code)]
            compiler => {
                panic!(
                    "The {:?} Compiler is not enabled. Enable it via the features",
                    compiler
                )
            }
        }
    }

    fn add_middlewares(&self, config: &mut dyn CompilerConfig) {
        for middleware in self.middlewares.iter() {
            config.push_middleware(middleware.clone());
        }
    }
}