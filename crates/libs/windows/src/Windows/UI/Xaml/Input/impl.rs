#[cfg(feature = "Foundation")]
pub trait ICommand_Impl: Sized {
    fn CanExecuteChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanExecuteChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanExecute(&self, parameter: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn Execute(&self, parameter: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ICommand";
}
#[cfg(feature = "Foundation")]
impl ICommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: isize>() -> ICommand_Vtbl {
        unsafe extern "system" fn CanExecuteChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanExecuteChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanExecuteChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCanExecuteChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn CanExecute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanExecute(::core::mem::transmute(&parameter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Execute(::core::mem::transmute(&parameter)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICommand, OFFSET>(),
            CanExecuteChanged: CanExecuteChanged::<Identity, Impl, OFFSET>,
            RemoveCanExecuteChanged: RemoveCanExecuteChanged::<Identity, Impl, OFFSET>,
            CanExecute: CanExecute::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommand as ::windows::core::Interface>::IID
    }
}
