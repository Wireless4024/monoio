use alloc::rc::Rc;
use std::{any::TypeId, cell::UnsafeCell, collections::VecDeque};

use crate::runtime::CURRENT;

/// A hook that run before runtime park.
pub trait PreParkHook: 'static {
    /// This will get called before runtime park.
    fn pre_park(&self);
    /// Get type id of this hook.
    fn hook_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

pub(crate) struct PreParkHooks {
    hooks: UnsafeCell<VecDeque<Rc<dyn PreParkHook>>>,
}

impl Default for PreParkHooks {
    #[inline]
    fn default() -> Self {
        Self {
            hooks: UnsafeCell::new(VecDeque::new()),
        }
    }
}

impl PreParkHooks {
    #[inline(always)]
    fn with_hook_mut<R>(
        &self,
        callback: impl FnOnce(&mut VecDeque<Rc<dyn PreParkHook>>) -> R,
    ) -> R {
        let hooks = unsafe { &mut *self.hooks.get() };
        callback(hooks)
    }

    pub(crate) fn run(&self) {
        self.with_hook_mut(|hooks| {
            let mut i = 0;
            while i < hooks.len() {
                let hook = &hooks[i];
                hook.pre_park();
                i += 1;
            }
        });
    }
}

/// Register hook that run before runtime park.
pub fn register_pre_park_hook<T: PreParkHook>(value: Rc<T>) {
    let hook = value as Rc<dyn PreParkHook>;

    CURRENT.with(|ctx| {
        ctx.pre_park_hooks
            .with_hook_mut(|hooks| hooks.push_back(hook));
    });
}

/// Unregister hook that run before runtime park.
pub fn unregister_pre_park_hook<T: PreParkHook>() -> Option<Rc<dyn PreParkHook>> {
    let typ = TypeId::of::<T>();
    CURRENT.with(|ctx| {
        ctx.pre_park_hooks.with_hook_mut(|hooks| {
            if let Some(hook_position) = hooks.iter().position(|hook| hook.hook_type_id() == typ) {
                return hooks.swap_remove_back(hook_position);
            }
            None
        })
    })
}
