extern mod xlib;
use std::ptr;
use std::option::Option;
use xlib::xlib::{Display, XSelectInput, XOpenDisplay, Window, XDefaultRootWindow};
use xlib::xlib::{XNextEvent, XEvent};
use std::libc::{size_t, malloc};

fn getDisplay() -> Option<*Display> {
    unsafe {
        let null = ptr::null();
        let display = XOpenDisplay(null);

        if (ptr::is_null(display)) {
            return None
        } else {
            return Some(display);
        }

    }
}

fn getRootWindow(display: *Display) -> Window {
    unsafe {
        return XDefaultRootWindow(display);
    }
}

fn setSelectInput(display: *Display, window: Window) {
    unsafe {
        let SubstructureNotifyMask = 1<<19;
        let SubstructureRedirectMask = 1<<20;
        let XSelectInputMask = SubstructureNotifyMask|SubstructureRedirectMask;
        XSelectInput(display, window, XSelectInputMask);
    }
}

fn main() {
    let maybeDisplay = getDisplay();
    match maybeDisplay {
        Some(display) => {
            println!("Got display {}", display);
            let root = getRootWindow(display);
            println!("Got root window {}", root);

            setSelectInput(display, root);
            println!("select input set!");

            unsafe {

                let xeventSize = std::mem::size_of::<*XEvent>() as size_t;
                let xevent = malloc(xeventSize) as *XEvent;
                println!("New event! {} with size {}", xevent, xeventSize);

                XNextEvent(display, xevent);
                println!("Next event! {}", xevent);
            }
        }
        None => println!("Error getting display")
    }
}
