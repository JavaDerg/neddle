use core::panicking::AssertKind::Match;
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use fxhash::FxBuildHasher;
use std::any::{Any, TypeId};

pub struct Context {
    map: DashMap<TypeId, Box<dyn Any>, FxBuildHasher>,
}

pub enum CtxResponse<'a, T> {
    Raw(T),
    Ref(Ref<'a, TypeId, Box<dyn Any>, FxBuildHasher>),
    OptRef(Option<Ref<'a, TypeId, Box<dyn Any>, FxBuildHasher>>),
}

trait Sealed {}
pub trait FromCtx: Sealed + Sized {
    fn from(ctx: &Context) -> CtxResponse<Self>;
}

impl Context {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

// Seal implementation
impl<T: FromCtx> Sealed for &T {}
impl<T: FromCtx> Sealed for Option<&T> {}

impl<T: 'static + FromCtx> FromCtx for &T {
    fn from(ctx: &Context) -> CtxResponse<Self> {
        let rf = ctx.map.get(&TypeId::of::<T>()).expect(&format!(
            "Type <{}> is required to be registered",
            std::any::type_name::<T>()
        ));
        CtxResponse::<&T>::Ref(rf)
    }
}

// TODO
impl<T: 'static + FromCtx> FromCtx for Option<&T> {
    fn from(ctx: &Context) -> CtxResponse<Self> {
        let rf = ctx.map.get(&TypeId::of::<T>());
        CtxResponse::<Option<&T>>::OptRef(rf)
    }
}
