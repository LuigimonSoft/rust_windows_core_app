#![windows_subsystem = "windows"]
use windows::{
    core::*,
    ApplicationModel::{Core::*, Package},
    Win32::{
        System::Com::*,
        UI::WindowsAndMessaging::{MessageBoxW, MB_ICONSTOP, MB_OK},
    },
    UI::Core::*, 
};

#[implement(IFrameworkViewSource)]
struct CoreApp();
#[implement(IFrameworkView)]
struct CoreAppView();

#[allow(non_snake_case)]
impl IFrameworkViewSource_Impl for CoreApp {
    fn CreateView(&self) -> Result<IFrameworkView> {
        Ok(CoreAppView().into())
    }
}

#[allow(non_snake_case)]
impl  IFrameworkView_Impl for CoreAppView {
    fn Initialize(&self, _: &Option<CoreApplicationView>) -> Result<()> {
        Ok(())
    }

    fn SetWindow(&self, _:&Option<CoreWindow>) -> Result<()> {
        Ok(())
    }

    fn Load(&self,entrypoint: &windows::core::HSTRING) ->  windows::core::Result<()> {
        Ok(())
    }

    fn Run(&self) ->  windows::core::Result<()> {
        let window = CoreWindow::GetForCurrentThread()?;
        window.Activate()?;

        let dispacher = window.Dispatcher()?;
        dispacher.ProcessEvents(CoreProcessEventsOption::ProcessUntilQuit)?;

        Ok(())
    }

    fn Uninitialize(&self) ->  windows::core::Result<()> {
        Ok(())
    }

}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED)?; 
        if let Err(e) = Package::Current() {
            MessageBoxW(None, w!("This sample must be registered (via register.cmd) and launched from Start."),w!("Error"), MB_ICONSTOP | MB_OK);
            return Err(e);
        }
    }

    let app: IFrameworkViewSource = CoreApp().into();
    CoreApplication::Run(&app)?;
    Ok(())
}
    

