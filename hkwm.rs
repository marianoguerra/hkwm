extern mod xlib;
use std::ptr;
use std::option::Option;
use xlib::xlib;
use xlib::{Display, XSelectInput, XOpenDisplay, Window, XDefaultRootWindow};
use xlib::{XNextEvent, Union__XEvent};

mod x;

fn getDisplay() -> Option<*mut Display> {
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

fn getRootWindow(display: *mut Display) -> Window {
    unsafe {
        return XDefaultRootWindow(display);
    }
}

fn setSelectInput(display: *mut Display, window: Window) {
    unsafe {
        let XSelectInputMask = x::SubstructureNotifyMask|x::SubstructureRedirectMask;
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
                let mut wa = xlib::XSetWindowAttributes {
                    background_pixmap: 0,
                    background_pixel: 0,
                    border_pixmap: 0,
                    border_pixel: 0,
                    bit_gravity: 0,
                    win_gravity: 0,
                    backing_store: 0,
                    backing_planes: 0,
                    backing_pixel: 0,
                    save_under: 0,
                    event_mask: 0,
                    do_not_propagate_mask: 0,
                    override_redirect: 0,
                    colormap: 0,
                    cursor: 0,
                };
                wa.cursor = xlib::XCreateFontCursor(display,
                                                x::cursorfont::XC_left_ptr);
                xlib::XChangeWindowAttributes(display, root,
                    x::CWCursor, &mut wa);
                xlib::XGrabKey(display, 53, x::Mod4Mask, root, 1, x::GrabModeAsync,
                    x::GrabModeAsync);
                xlib::XGrabKey(display, 45, x::Mod4Mask, root, 1, x::GrabModeAsync,
                    x::GrabModeAsync);

                let mut xevent = Union__XEvent{ data: [0, ..24] };
                loop {
                    XNextEvent(display, ptr::to_mut_unsafe_ptr(&mut xevent));
                    let eventType = *xevent._type();

                    match eventType {
                        x::MapRequest => {
                            let screen = xlib::XDefaultScreen(display);
                            let width = xlib::XDisplayWidth(display, screen);
                            let height = xlib::XDisplayHeight(display, screen);

                            println!("Map request screen {} width {} height {}",
                                screen, width, height);

                            let event = *xevent.xmaprequest();
                            xlib::XMapWindow(display, event.window);
                            xlib::XSetInputFocus(display, event.window,
                                x::RevertToPointerRoot, x::CurrentTime);
                            xlib::XMoveResizeWindow(display, event.window, 0, 0,
                                width as u32, height as u32);
                        }
                        x::KeyRelease => {
                            let event = *xevent.xkey();
                            println!("Key {}", event.keycode);
                            if (event.keycode == 45) {
                                    let mut focus: Window = 0;
                                    let mut revert: i32 = 0;
                                    xlib::XGetInputFocus(display, &mut focus,
                                        &mut revert);
                                    xlib::XDestroyWindow(display, focus);
                            }
                        }
                        _ => {
                            println!("Unknown Event Type {}", eventType);
                        }
                    }
                }
            }
        }
        None => println!("Error getting display")
    }
}
